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

use crate::diesel;
use crate::model;
use crate::schema;
use crate::utils::PgPool;

use crate::appointments::appointment_giver_server::{AppointmentGiver, AppointmentGiverServer};
use crate::appointments::{
    BookAppointmentReply, BookAppointmentRequest, CancelAppointmentReply, CancelAppointmentRequest,
    GetAppointmentReply, GetAppointmentRequest, GetAvailabilityReply, GetAvailabilityRequest,
    GetLocationsReply, GetLocationsRequest,
};

pub struct MyAppointmentGiver {
    db: PgPool,
}

impl MyAppointmentGiver {
    fn new(pool: PgPool) -> MyAppointmentGiver {
        MyAppointmentGiver { db: pool }
    }
}

#[tonic::async_trait]
impl AppointmentGiver for MyAppointmentGiver {
    async fn get_locations(
        &self,
        _request: Request<GetLocationsRequest>,
    ) -> Result<Response<GetLocationsReply>, Status> {
        use diesel::RunQueryDsl;
        use schema::locations;
        let result = locations::table
            .load::<model::Location>(&self.db.get().unwrap())
            .unwrap();

        Ok(tonic::Response::new(GetLocationsReply {
            location: result.iter().map(|x| x.into()).collect(),
        }))
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

pub async fn server(db_pool: PgPool, addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let addr_parsed = addr.parse()?;

    let appointment_giver = MyAppointmentGiver::new(db_pool);

    Server::builder()
        .add_service(AppointmentGiverServer::new(appointment_giver))
        .serve(addr_parsed)
        .await?;
    Ok(())
}
