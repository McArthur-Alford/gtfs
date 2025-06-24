use csv::{ReaderBuilder, StringRecord};
use rayon::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

// TODO: Use rayon here.
// Task for each file.
// For each task, do a par iter over the csv readers lines.
// Deserialize them all excessively fast.

#[derive(Debug, Deserialize)]
pub struct Agency {
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: String,
    pub agency_phone: String,
}

#[derive(Debug, Deserialize)]
pub struct Calendar {
    pub service_id: String,
    pub monday: u8,
    pub tuesday: u8,
    pub wednesday: u8,
    pub thursday: u8,
    pub friday: u8,
    pub saturday: u8,
    pub sunday: u8,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct CalendarDate {
    pub service_id: String,
    pub date: String,
    pub exception_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct FeedInfo {
    pub feed_publisher_name: String,
    pub feed_publisher_url: String,
    pub feed_lang: String,
    pub feed_start_date: String,
    pub feed_end_date: String,
}

#[derive(Debug, Deserialize)]
pub struct Route {
    pub route_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: Option<String>,
    pub route_type: u8,
    pub route_url: String,
    pub route_color: String,
    pub route_text_color: String,
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    pub shape_id: String,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_pt_sequence: u32,
}

#[derive(Debug, Deserialize)]
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: u32,
    pub pickup_type: u8,
    pub drop_off_type: u8,
}

#[derive(Debug, Deserialize)]
pub struct Stop {
    pub stop_id: String,
    pub stop_code: String,
    pub stop_name: String,
    pub stop_desc: Option<String>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: String,
    pub stop_url: String,
    pub location_type: u8,
    pub parent_station: Option<String>,
    pub platform_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Trip {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: String,
    pub direction_id: u8,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
}

// fn () -> Result<(), Box<dyn Error>> {
//     let file = File::open("./seq_gtfs/stop_times.txt")?;
//     let mut rdr = ReaderBuilder::new()
//         .has_headers(true)
//         .from_reader(BufReader::new(file));

//     let headers = rdr.headers()?.clone();
//     let records: Vec<StringRecord> = rdr.records().filter_map(Result::ok).collect();

//     let parsed: Vec<StopTime> = records
//         .into_par_iter()
//         .filter_map(|record| record.deserialize(Some(&headers)).ok())
//         .collect();

//     for stop_time in &parsed {
//         println!("{:?}", stop_time);
//     }

//     Ok(())
// }
