/*
	appointments: A privacy respecting appointment booking system
	Copyright (C) 2021 Morgan Hill

	This program is free software: you can redistribute it and/or modify
	it under the terms of the GNU Affero General Public License as published
	by the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU Affero General Public License for more details.

	You should have received a copy of the GNU Affero General Public License
	along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

CREATE TABLE locations (
	id UUID PRIMARY KEY NOT NULL,
	name TEXT NOT NULL,
	address TEXT NOT NULL,
	phone TEXT NOT NULL
);

CREATE TABLE slots (
	id UUID PRIMARY KEY NOT NULL,
	start_time TIMESTAMP NOT NULL,
	end_time TIMESTAMP NOT NULL,
	location UUID NOT NULL REFERENCES locations (id),
	capacity INTEGER NOT NULL,
	CONSTRAINT start_time_location UNIQUE (start_time, location)
);

CREATE INDEX idx_slots_start_time_location ON slots (start_time, location);

CREATE TABLE appointments (
	id UUID PRIMARY KEY NOT NULL,
	slot UUID NOT NULL REFERENCES slots (id),
	location UUID NOT NULL REFERENCES locations (id),
	human_id integer NOT NULL CHECK (human_id >= 0 AND human_id <= 60466176),
	CONSTRAINT location_hummanId_unique UNIQUE (location, human_id)
);

CREATE INDEX idx_appointments_slot ON appointments (slot);
CREATE INDEX idx_appointments_location ON appointments (location);
CREATE INDEX idx_appointments_human_id ON appointments (human_id);

CREATE VIEW slots_view AS
	SELECT
		slots.id AS id, 
		slots.start_time AS start_time, 
		slots.end_time AS end_time, 
		slots.location AS location_id, 
		locations.name AS location_name, 
		slots.capacity AS capacity,
		slots.capacity - (SELECT COUNT(*) FROM appointments WHERE slot = slots.id)::INTEGER AS availability
	FROM slots
	INNER JOIN locations ON slots.location = locations.id;
