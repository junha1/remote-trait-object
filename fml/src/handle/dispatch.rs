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

use super::table::ServiceObjectTable;
use super::PortId;
use super::{HandleInstance, MethodId, Service, ServiceObjectId, TraitId};
use crate::context;
use parking_lot::RwLock;
use std::sync::Arc;

// 1. PortDispatcher: Dispatch given packet to the target instance.
// This process is general over traits.
// 2. ServiceDispatcher: Dispatch given packet (which has been dispatched by PortDispatcher)
// to the target method. This process is very specific to each trait,
// and is generated by the proc macro.

pub trait ServiceDispatcher: Send + Sync {
    fn dispatch(&self, method: MethodId, arguments: &[u8], return_buffer: std::io::Cursor<&mut Vec<u8>>);
}

pub struct PortDispatcher {
    service_table: RwLock<ServiceObjectTable>,
    id: PortId,
}

impl PortDispatcher {
    pub fn new(id: PortId, size: usize) -> Self {
        PortDispatcher {
            service_table: RwLock::new(ServiceObjectTable::new(size)),
            id,
        }
    }

    pub fn get_id(&self) -> PortId {
        self.id
    }

    pub fn dispatch(
        &self,
        handle: ServiceObjectId,
        method: MethodId,
        arguments: &[u8],
        return_buffer: std::io::Cursor<&mut Vec<u8>>,
    ) {
        #[cfg(fml_statistics)]
        {
            crate::statistics::DISPATCH_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let service_object = self.service_table.read().get(handle.index as usize);
        // NOTE: You must drop the ReadGuard before dispatch (if not deadlock)
        service_object.dispatch(method, arguments, return_buffer);
    }
}

pub fn register(port_id: PortId, mut handle_to_register: Arc<dyn Service>) -> HandleInstance {
    #[cfg(fml_statistics)]
    {
        crate::statistics::CREATE_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    let context = context::global::get();
    let port_table = context.read();

    Arc::get_mut(&mut handle_to_register).unwrap().get_handle_mut().port_id_exporter = port_id;
    Arc::get_mut(&mut handle_to_register).unwrap().get_handle_mut().port_id_importer =
        port_table.map.get(&port_id).unwrap().1;

    let port = &port_table.map.get(&port_id).expect("PortTable corrupted").2;
    port.dispatcher_get().service_table.write().create(handle_to_register).get_handle().careful_clone()
}

pub fn delete(port_id: PortId, handle: ServiceObjectId) {
    let context = context::global::get();
    let port_table = context.read();

    let port = &port_table.map.get(&port_id).expect("PortTable corrupted").2;
    port.dispatcher_get().service_table.write().remove(handle.index as usize)
}
