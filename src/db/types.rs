//! DB Types
//!
//! Types for database operations.
//! Should directly map to the schema tables.

use sqlx::FromRow;

/// Representation of agency table rows
#[derive(Debug, FromRow)]
pub struct Agency {
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
}

/// Representation of stops table rows
#[derive(Debug, FromRow)]
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
#[derive(Debug, FromRow)]
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
#[derive(Debug, FromRow)]
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
#[derive(Debug, FromRow)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: Option<chrono::Duration>,
    pub departure_time: chrono::Duration,
    pub stop_id: String,
    pub stop_sequence: i32,
    pub pickup_type: i32,
    pub drop_off_type: i32,
}

/// Representation of calendar table rows
#[derive(Debug, FromRow)]
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
#[derive(Debug, FromRow)]
pub struct CalendarDate {
    pub service_id: String,
    pub date: i64,
    pub exception_type: i32,
}

/// Representation of shapes table rows
#[derive(Debug, FromRow)]
pub struct Shape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: i32,
}

/// Representation of feed_info table rows
#[derive(Debug, FromRow)]
pub struct FeedInfo {
    pub feed_publisher_name: String,
    pub feed_publisher_url: String,
    pub feed_lang: Option<String>,
    pub feed_start_date: Option<i64>,
    pub feed_end_date: Option<i64>,
}
