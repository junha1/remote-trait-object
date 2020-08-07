// Copyright 2020 Kodebox, Inc.
// This file is part of CodeChain.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::*;
use crate::packet::Packet;
use crate::raw_exchange::HandleToExchange;
use crate::service::{MethodId, SerdeFormat};

/// Proxy service will carry this.
#[derive(Debug)]
pub struct Handle {
    pub id: ServiceObjectId,
    pub port: Weak<dyn Port>,
}

impl Handle {
    pub fn new(imported_id: HandleToExchange, port: Weak<dyn Port>) -> Self {
        Handle {
            id: imported_id.0,
            port,
        }
    }
}

impl Handle {
    /// This method is the core of Handle, which serves as a "call stub" for the service trait's method.
    /// It carries out user's remote call in a generic way.
    /// Invoking this method is role of the macro, by putting appropriate instantiation of this generic
    /// for each service trait's method, according to the method signature of each.
    pub fn call<F: SerdeFormat, S: serde::Serialize, D: serde::de::DeserializeOwned>(
        &self,
        method: MethodId,
        args: &S,
    ) -> D {
        super::serde_support::port_thread_local::set_port(self.port.clone());
        let args = F::to_vec(args).unwrap();
        let packet = Packet::new_request(self.id, method, &args);
        let response = self.port.upgrade().unwrap().call(packet.view());
        let result = F::from_slice(response.data()).unwrap();
        super::serde_support::port_thread_local::remove_port();
        result
    }
}

impl Drop for Handle {
    /// Dropping handle will be signaled to the exporter (_delete request_), so that it can remove the service object as well.
    fn drop(&mut self) {
        self.port
            .upgrade()
            .expect("You must drop the proxy object before the RTO context is dropped")
            .delete_request(self.id);
    }
}
