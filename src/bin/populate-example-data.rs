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
extern crate appointments;
extern crate diesel;
extern crate uuid;

use crate::diesel::prelude::*;
use crate::diesel::{insert_into, pg::PgConnection, r2d2::ConnectionManager, r2d2::Pool};

use std::env;

use appointments::model::{Location, Slot};
use appointments::schema::locations::dsl::*;
use appointments::schema::slots::dsl::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).unwrap();
    let db = pool.get().unwrap();
    let example_locations: Vec<Location> = vec![
        Location {
            id: uuid::Uuid::new_v4(),
            name: "Example 1".into(),
            address: "Example street 123, Exampleton, Exampleshire, EX44MP".into(),
            phone: "123456789".into(),
        },
        Location {
            id: uuid::Uuid::new_v4(),
            name: "Example 2".into(),
            address: "Example street 124, Exampleton, Exampleshire, EX44MP".into(),
            phone: "123456789".into(),
        },
    ];
    insert_into(locations)
        .values(&example_locations)
        .execute(&db)?;
    use std::time::{Duration, SystemTime};
    let now = SystemTime::now();
    for l in example_locations {
        let times = (1..1000).map(|x| {
            (
                now.checked_add(Duration::from_secs((x - 1) * 10 * 60))
                    .unwrap(),
                now.checked_add(Duration::from_secs(x * 10 * 60)).unwrap(),
            )
        });
        let example_slots: Vec<Slot> = times
            .map(|t| Slot {
                id: uuid::Uuid::new_v4(),
                location: l.id,
                start_time: t.0,
                end_time: t.1,
                capacity: 100,
            })
            .collect();
        insert_into(slots).values(&example_slots).execute(&db)?;
    }
    Ok(())
}
