//! DB Types
//!
//! Types for database operations.
//! Should directly map to the schema tables.

use sqlx::{FromRow, PgConnection, postgres::types::PgInterval};

use crate::db::{self, Db, queries::*};

pub trait InsertDB: Sized + Send + Sync {
    fn insert(
        &self,
        pool: &mut PgConnection,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + std::marker::Send;
}

/// Representation of agency table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Agency {
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
}

/// Representation of stops table rows
#[derive(Debug, FromRow, PartialEq)]
pub struct Stop {
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_lat: Option<f64>,
    pub stop_lon: Option<f64>,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: Option<i32>,
    pub parent_station: Option<String>,
    pub platform_code: Option<String>,
}

/// Representation of routes table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Route {
    pub route_id: String,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_desc: Option<String>,
    pub route_type: i32,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
}

/// Representation of trips table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub direction_id: Option<bool>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
}

/// Representation of stop_times table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: Option<PgInterval>,
    pub departure_time: PgInterval,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub pickup_type: i32,
    pub drop_off_type: i32,
}

/// Representation of calendar table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct Calendar {
    pub service_id: String,
    pub monday: bool,
    pub tuesday: bool,
    pub wednesday: bool,
    pub thursday: bool,
    pub friday: bool,
    pub saturday: bool,
    pub sunday: bool,
    pub start_date: i64,
    pub end_date: i64,
}

/// Representation of calendar_date table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct CalendarDate {
    pub service_id: String,
    pub date: i64,
    pub exception_type: i32,
}

/// Representation of shapes table rows
#[derive(Debug, FromRow, PartialEq)]
pub struct Shape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: i32,
}

/// Representation of feed_info table rows
#[derive(Debug, FromRow, PartialEq, Eq)]
pub struct FeedInfo {
    pub feed_publisher_name: String,
    pub feed_publisher_url: String,
    pub feed_lang: Option<String>,
    pub feed_start_date: Option<i32>,
    pub feed_end_date: Option<i32>,
}

impl InsertDB for Agency {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_agency(self, db).await
    }
}
impl InsertDB for Stop {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_stop(self, db).await
    }
}
impl InsertDB for Route {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_route(self, db).await
    }
}
impl InsertDB for Trip {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_trip(self, db).await
    }
}
impl InsertDB for StopTime {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_stop_time(self, db).await
    }
}

impl InsertDB for Calendar {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_calendar(self, db).await
    }
}

impl InsertDB for CalendarDate {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_calendar_date(self, db).await
    }
}

impl InsertDB for Shape {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_shape(self, db).await
    }
}
impl InsertDB for FeedInfo {
    async fn insert(&self, db: &mut PgConnection) -> Result<(), sqlx::Error> {
        insert_feed_info(self, db).await
    }
}
