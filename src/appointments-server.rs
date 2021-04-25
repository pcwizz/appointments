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
use tonic::{transport::Server, Request, Response, Status};

use appointments::appointment_giver_server::{AppointmentGiver, AppointmentGiverServer};
use appointments::{
    BookAppointmentReply, BookAppointmentRequest, CancelAppointmentReply, CancelAppointmentRequest,
    GetAppointmentReply, GetAppointmentRequest, GetAvailabilityReply, GetAvailabilityRequest,
    GetLocationsReply, GetLocationsRequest,
};

pub mod appointments {
    tonic::include_proto!("appointments");
}

#[derive(Debug, Default)]
pub struct MyAppointmentGiver {}

#[tonic::async_trait]
impl AppointmentGiver for MyAppointmentGiver {
    async fn get_locations(
        &self,
        request: Request<GetLocationsRequest>,
    ) -> Result<Response<GetLocationsReply>, Status> {
        unimplemented!()
    }

    async fn get_availability(
        &self,
        request: Request<GetAvailabilityRequest>,
    ) -> Result<Response<GetAvailabilityReply>, Status> {
        unimplemented!()
    }

    async fn book_appointment(
        &self,
        request: Request<BookAppointmentRequest>,
    ) -> Result<Response<BookAppointmentReply>, Status> {
        unimplemented!()
    }

    async fn get_appointment(
        &self,
        request: Request<GetAppointmentRequest>,
    ) -> Result<Response<GetAppointmentReply>, Status> {
        unimplemented!()
    }

    async fn cancel_appointment(
        &self,
        request: Request<CancelAppointmentRequest>,
    ) -> Result<Response<CancelAppointmentReply>, Status> {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let appointmentGiver = MyAppointmentGiver::default();

    Server::builder()
        .add_service(AppointmentGiverServer::new(appointmentGiver))
        .serve(addr)
        .await?;

    Ok(())
}
