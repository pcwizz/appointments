// appointments: A privacy respecting appointment booking system
// Copyright (C) 2021 Morgan Hill

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[macro_use]
extern crate diesel;
extern crate uuid;

pub mod giver;
#[cfg(test)]
mod giver_test;
pub mod model;
#[cfg(test)]
mod model_test;
pub mod schema;
pub mod utils;

pub mod appointments {
    tonic::include_proto!("appointments");
}