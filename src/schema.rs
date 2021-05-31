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

table! {
    appointments (id) {
        id -> Uuid,
        slot -> Uuid,
        location -> Uuid,
        human_id -> Int4,
    }
}

table! {
    locations (id) {
        id -> Uuid,
        name -> Text,
        address -> Text,
        phone -> Text,
    }
}

table! {
    slots (id) {
        id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        location -> Uuid,
        capacity -> Int4,
    }
}

table! {
    slots_view (id) {
        id -> Uuid,
        start_time -> Timestamp,
        end_time -> Timestamp,
        location_id -> Uuid,
        location_name -> Text,
        capacity -> Int4,
        availability -> Int4,
    }
}

joinable!(appointments -> locations (location));
joinable!(appointments -> slots (slot));
joinable!(slots -> locations (location));

allow_tables_to_appear_in_same_query!(appointments, locations, slots);
