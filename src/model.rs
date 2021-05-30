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

#[derive(Queryable)]
pub struct Location {
    pub id: uuid::Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
}

impl From<&Location> for crate::appointments::Location {
    fn from(item: &Location) -> Self {
        crate::appointments::Location {
            id: Some(crate::appointments::Uuid {
                uuid: item.id.to_string(),
            }),
            name: item.name.clone(),
            address: item.address.clone(),
            phone: item.phone.clone(),
        }
    }
}

impl From<&crate::appointments::Location> for Location {
    fn from(item: &crate::appointments::Location) -> Self {
        Location {
            id: item
                .id
                .as_ref()
                .expect("location id missing")
                .uuid
                .parse()
                .expect("malformed location id"),
            name: item.name.clone(),
            address: item.address.clone(),
            phone: item.phone.clone(),
        }
    }
}
