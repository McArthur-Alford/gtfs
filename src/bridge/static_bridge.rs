use std::time::Duration;

use crate::db;
use anyhow::anyhow;
use anyhow::{Context, Result};
use chrono::NaiveDate;
use futures::future::join_all;
use sqlx::postgres::types::PgInterval;
use tracing::{error, instrument};

#[derive(Debug)]
pub struct GtfsDbModel {
    pub agencies: Vec<db::types::Agency>,
    pub stops: Vec<db::types::Stop>,
    pub routes: Vec<db::types::Route>,
    pub trips: Vec<db::types::Trip>,
    pub stop_times: Vec<db::types::StopTime>,
    pub calendar: Vec<db::types::Calendar>,
    pub calendar_dates: Vec<db::types::CalendarDate>,
    pub shapes: Vec<db::types::Shape>,
    pub feed_info: Vec<db::types::FeedInfo>,
}

pub trait ToDB<T> {
    async fn to_db(self) -> Result<T>;
}

impl ToDB<bool> for gtfs_structures::DirectionType {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<bool> {
        Ok(match self {
            gtfs_structures::DirectionType::Outbound => false,
            gtfs_structures::DirectionType::Inbound => true,
        })
    }
}

impl ToDB<i32> for gtfs_structures::LocationType {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<i32> {
        Ok(match self {
            gtfs_structures::LocationType::StopPoint => 0,
            gtfs_structures::LocationType::StopArea => 1,
            gtfs_structures::LocationType::StationEntrance => 2,
            gtfs_structures::LocationType::GenericNode => 3,
            gtfs_structures::LocationType::BoardingArea => 4,
            gtfs_structures::LocationType::Unknown(i) => i,
        }
        .into())
    }
}

impl ToDB<i32> for gtfs_structures::RouteType {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<i32> {
        Ok(match self {
            gtfs_structures::RouteType::Tramway => 0,
            gtfs_structures::RouteType::Subway => 1,
            gtfs_structures::RouteType::Rail => 2,
            gtfs_structures::RouteType::Bus => 3,
            gtfs_structures::RouteType::Ferry => 4,
            gtfs_structures::RouteType::CableCar => 5,
            gtfs_structures::RouteType::Gondola => 6,
            gtfs_structures::RouteType::Funicular => 7,
            gtfs_structures::RouteType::Coach => 8,
            gtfs_structures::RouteType::Air => 9,
            gtfs_structures::RouteType::Taxi => 10,
            gtfs_structures::RouteType::Other(i) => i,
        }
        .into())
    }
}

impl ToDB<PgInterval> for u32 {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<PgInterval> {
        Ok(
            match PgInterval::try_from(Duration::from_secs(self.into())) {
                Ok(interval) => interval,
                Err(_) => return Err(anyhow!("Failed to parse PgInterval")),
            },
        )
    }
}

impl ToDB<i32> for gtfs_structures::PickupDropOffType {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<i32> {
        // 0..=3 from the spec https://gtfs.org/documentation/schedule/reference/#stop_timestxt
        Ok(match self {
            gtfs_structures::PickupDropOffType::Regular => 0,
            gtfs_structures::PickupDropOffType::NotAvailable => 1,
            gtfs_structures::PickupDropOffType::ArrangeByPhone => 2,
            gtfs_structures::PickupDropOffType::CoordinateWithDriver => 3,
            gtfs_structures::PickupDropOffType::Unknown(i) => i,
        }
        .into())
    }
}

impl ToDB<i32> for NaiveDate {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<i32> {
        // GTFS stores things as yyyymmdd as a numeric(8) type in postgres.
        // This is annoying and i had to change it to an integer.
        // Lets hope it doesn't backfire?
        Ok(self
            .format("%Y%m%d")
            .to_string()
            .parse::<i32>()
            .context("Failed to format NaiveDate as i32 (yyyymmdd)")?)
    }
}

impl ToDB<i32> for gtfs_structures::Exception {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<i32> {
        Ok(match self {
            gtfs_structures::Exception::Added => 1,
            gtfs_structures::Exception::Deleted => 2,
        })
    }
}

impl ToDB<db::types::Trip> for gtfs_structures::RawTrip {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::Trip> {
        Ok(db::types::Trip {
            trip_id: self.id,
            service_id: self.service_id,
            route_id: self.route_id,
            trip_headsign: self.trip_headsign,
            direction_id: match self.direction_id.map(gtfs_structures::DirectionType::to_db) {
                Some(fut) => Some(fut.await?),
                None => None,
            },
            block_id: self.block_id,
            shape_id: self.shape_id,
        })
    }
}

impl ToDB<db::types::StopTime> for gtfs_structures::RawStopTime {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::StopTime> {
        Ok(db::types::StopTime {
            trip_id: self.trip_id,
            arrival_time: match self.arrival_time.map(u32::to_db) {
                Some(fut) => Some(fut.await?),
                None => None,
            },
            departure_time: self
                .departure_time
                .context("Missing departure time")?
                .to_db()
                .await?,
            stop_id: self.stop_id,
            stop_sequence: self.stop_sequence.try_into()?,
            pickup_type: self.pickup_type.to_db().await?,
            drop_off_type: self.drop_off_type.to_db().await?,
        })
    }
}

impl ToDB<db::types::Agency> for gtfs_structures::Agency {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::Agency> {
        Ok(db::types::Agency {
            agency_name: self.name,
            agency_url: self.url,
            agency_timezone: self.timezone,
            agency_lang: self.lang,
            agency_phone: self.phone,
        })
    }
}

impl ToDB<db::types::Stop> for gtfs_structures::Stop {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::Stop> {
        Ok(db::types::Stop {
            stop_id: self.id,
            stop_code: self.code,
            stop_name: self.name,
            stop_desc: self.description,
            stop_lat: self.latitude,
            stop_lon: self.longitude,
            zone_id: self.zone_id,
            stop_url: self.url,
            location_type: Some(self.location_type.to_db().await?),
            parent_station: self.parent_station,
            platform_code: self.platform_code,
        })
    }
}

impl ToDB<db::types::Route> for gtfs_structures::Route {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::Route> {
        Ok(db::types::Route {
            route_id: self.id,
            route_short_name: self.short_name,
            route_long_name: self.long_name,
            route_desc: self.desc,
            route_type: self.route_type.to_db().await?,
            route_url: self.url,
            route_color: Some(self.color.to_string()),
            route_text_color: Some(self.text_color.to_string()),
        })
    }
}

impl ToDB<db::types::Calendar> for gtfs_structures::Calendar {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::Calendar> {
        Ok(db::types::Calendar {
            service_id: self.id,
            monday: self.monday,
            tuesday: self.tuesday,
            wednesday: self.wednesday,
            thursday: self.thursday,
            friday: self.friday,
            saturday: self.saturday,
            sunday: self.sunday,
            start_date: self.start_date.to_db().await?.into(),
            end_date: self.end_date.to_db().await?.into(),
        })
    }
}

impl ToDB<db::types::CalendarDate> for gtfs_structures::CalendarDate {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::CalendarDate> {
        Ok(db::types::CalendarDate {
            service_id: self.service_id,
            date: self.date.to_db().await?.into(),
            exception_type: self.exception_type.to_db().await?,
        })
    }
}

impl ToDB<db::types::Shape> for gtfs_structures::Shape {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::Shape> {
        Ok(db::types::Shape {
            shape_id: self.id,
            shape_pt_lat: self.latitude,
            shape_pt_lon: self.longitude,
            shape_pt_sequence: self.sequence.try_into()?,
        })
    }
}

impl ToDB<db::types::FeedInfo> for gtfs_structures::FeedInfo {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<db::types::FeedInfo> {
        Ok(db::types::FeedInfo {
            feed_publisher_name: self.name,
            feed_publisher_url: self.url,
            feed_lang: Some(self.lang),
            feed_start_date: match self.start_date.map(NaiveDate::to_db) {
                Some(st) => Some(st.await?),
                None => None,
            },
            feed_end_date: match self.end_date.map(NaiveDate::to_db) {
                Some(ed) => Some(ed.await?),
                None => None,
            },
        })
    }
}

async fn convert_vec<T, U, It>(items: It) -> Vec<U>
where
    T: ToDB<U> + std::fmt::Debug + Send,
    U: Send,
    It: IntoIterator<Item = T>,
{
    let futures = items.into_iter().map(|item| async move {
        match item.to_db().await {
            Ok(value) => Some(value),
            Err(err) => {
                error!(error = ?err, "Failed to convert item to db representation");
                None
            }
        }
    });

    join_all(futures).await.into_iter().flatten().collect()
}

impl ToDB<GtfsDbModel> for gtfs_structures::RawGtfs {
    #[instrument(skip(self))]
    async fn to_db(self) -> Result<GtfsDbModel> {
        Ok(GtfsDbModel {
            agencies: convert_vec(self.agencies?).await,
            stops: convert_vec(self.stops?).await,
            routes: convert_vec(self.routes?).await,
            trips: convert_vec(self.trips?).await,
            stop_times: convert_vec(self.stop_times?).await,
            calendar: convert_vec(self.calendar.unwrap_or(Ok(vec![]))?).await,
            calendar_dates: convert_vec(self.calendar_dates.unwrap_or(Ok(vec![]))?).await,
            shapes: convert_vec(self.shapes.unwrap_or(Ok(vec![]))?).await,
            feed_info: convert_vec(self.feed_info.unwrap_or(Ok(vec![]))?).await,
        })
    }
}
