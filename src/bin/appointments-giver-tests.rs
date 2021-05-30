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

use appointments::appointments::{
    appointment_giver_client::AppointmentGiverClient, get_availability_reply::Availability,
    Appointment, BookAppointmentRequest, CancelAppointmentRequest, GetAppointmentRequest,
    GetAvailabilityRequest, GetLocationsRequest, TimeSpan,
};
use std::{
    env,
    error::Error,
    fmt,
    time::{Duration, SystemTime},
};
use tonic::transport::Channel;

#[derive(Debug)]
struct TestError {
    mesasage: &'static str,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mesasage)
    }
}

impl Error for TestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl TestError {
    fn new(m: &'static str) -> Self {
        TestError { mesasage: m }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!();
        return Err(
            TestError::new("Expected one argument: the address of the appointment giver").into(),
        );
    }

    let ch = Channel::from_shared(args[1].clone())?.connect().await?;
    let mut client = AppointmentGiverClient::new(ch);

    let happy_path = test_happy_path(&mut client);

    happy_path.await
}

async fn test_happy_path(
    client: &mut AppointmentGiverClient<tonic::transport::Channel>,
) -> Result<(), Box<dyn Error>> {
    println!("Testing the happy path");
    let locations = client
        .get_locations(GetLocationsRequest {})
        .await?
        .into_inner();
    println!("{:?}", locations);
    let availability = client
        .get_availability(GetAvailabilityRequest {
            location: locations
                .location
                .into_iter()
                .map(|l| l.id.expect("location missing ID"))
                .collect(),
            timespan: Some(TimeSpan {
                start: Some(SystemTime::now().into()),
                end: Some(
                    SystemTime::now()
                        .checked_add(Duration::from_secs(60 ^ 2 * 24 * 7))
                        .expect("SystemTime overflowed")
                        .into(),
                ),
            }),
        })
        .await?
        .into_inner();
    println!("{:?}", availability);

    let slots = availability
        .availability
        .unwrap_or(Availability { slot: vec![] });
    if slots.slot.len() == 0 {
        return Ok(());
    }
    let mut my_appointment = Appointment {
        human_id: 0,
        slot: None,
        uuid: None,
    };
    for slot in slots.slot {
        let appointment = client
            .book_appointment(BookAppointmentRequest {
                slot: Some(slot.to_owned()),
            })
            .await?
            .into_inner();
        if let Some(a) = appointment.appointment {
            my_appointment = a;
            break;
        }
    }
    let location_id = my_appointment
        .slot
        .clone()
        .expect("no slot in appointment")
        .location_id;

    let got_appointment = client
        .get_appointment(GetAppointmentRequest {
            appointment: my_appointment.human_id,
            location: location_id,
        })
        .await?
        .into_inner()
        .appointment
        .expect("human id didn't retrieve an appointment");

    assert_eq!(my_appointment, got_appointment);

    if !client
        .cancel_appointment(CancelAppointmentRequest {
            appointment: Some(got_appointment),
        })
        .await?
        .into_inner()
        .okay
    {
        panic!("could not cancel appointment");
    }
    println!("Happy path passed.");

    Ok(())
}
