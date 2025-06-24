use std::env::var;

pub fn db_url() -> String {
    var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub const REALTIME_URL: &str = "https://gtfsrt.api.translink.com.au/api/realtime";
pub const REALTIME_ENDPOINTS: [&str; 3] = ["SEQ/TripUpdates", "SEQ/VehiclePositions", "SEQ/alerts"];
pub const STATIC_URL: &str = "https://gtfsrt.api.translink.com.au/GTFS/SEQ_GTFS.zip";

pub fn realtime_urls() -> Vec<String> {
    REALTIME_ENDPOINTS
        .iter()
        .map(|ep| format!("{}/{}", REALTIME_URL, ep))
        .collect()
}
