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

// This module provides required components in FML while expanding FML macro.

pub mod impl_env {
    pub use crate::service::dispatch::ServiceDispatcher;
    pub use crate::service::Service;
    pub use crate::service::{DispatchService, HandleInstance, IdOfService, MethodId, TraitId};
    pub use std::sync::Arc;
}

pub use crate::service::HandleInstance;
pub use crate::service::SArc;
