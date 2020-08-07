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

use super::Dispatch;
use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// This represents transportable identifier of the service object
/// and should be enough to construct a handle along with the pointer to the port
/// which this service belong to
#[derive(PartialEq, Serialize, Deserialize, Debug, Clone, Copy)]
pub struct HandleToExchange(pub(crate) ServiceObjectId);

pub struct Skeleton {
    pub(crate) raw: Arc<dyn Dispatch>,
}

impl Clone for Skeleton {
    /// Clones a `Skeleton`, while sharing the actual single service object.
    ///
    /// This is useful when you want to export a single service object to multiple connections.
    fn clone(&self) -> Self {
        Self {
            raw: Arc::clone(&self.raw),
        }
    }
}

impl Skeleton {
    pub fn new<T: ?Sized + Service>(service: impl IntoSkeleton<T>) -> Self {
        service.into_skeleton()
    }
}

// This belongs to macro_env
pub fn create_skeleton(raw: Arc<dyn Dispatch>) -> Skeleton {
    Skeleton {
        raw,
    }
}

// These traits are associated with some specific service trait.
// These tratis will be implement by `dyn ServiceTrait` where `T = dyn ServiceTrait` as well.
// Macro will implement this trait with the target(expanding) service trait.

/// Unused T is for avoiding violation of the orphan rule
/// T will be local type for the crate, and that makes it possible to
/// ```ignore
/// impl IntoSkeleton<dyn MyService> for Arc<dyn MyService>
/// ```
pub trait IntoSkeleton<T: ?Sized + Service> {
    fn into_skeleton(self) -> Skeleton;
}

/// Unused T is for avoiding violation of the orphan rule, like `IntoSkeleton`
pub trait ImportProxy<T: ?Sized + Service>: Sized {
    fn import_proxy(port: Weak<dyn Port>, handle: HandleToExchange) -> Self;
}

// These functions are utilities for the generic traits above

pub fn export_service_into_handle(context: &crate::context::Context, service: Skeleton) -> HandleToExchange {
    context.get_port().upgrade().unwrap().register_service(service.raw)
}

pub fn import_service_from_handle<T: ?Sized + Service, P: ImportProxy<T>>(
    context: &crate::context::Context,
    handle: HandleToExchange,
) -> P {
    P::import_proxy(context.get_port(), handle)
}
