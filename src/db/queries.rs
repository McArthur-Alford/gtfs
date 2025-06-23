//! DB Queries
//!
//! A whole bunch of internal queries for the db.
//! All the SQL should be in here.

use std::path::Path;

use super::types::*;
use chrono::TimeDelta;
use sqlx::PgPool;

pub async fn insert_agency(agency: &Agency, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO agency (agency_name, agency_url, agency_timezone, agency_lang, agency_phone)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        agency.agency_name,
        agency.agency_url,
        agency.agency_timezone,
        agency.agency_lang,
        agency.agency_phone
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_stop(stop: &Stop, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO stops (
            stop_id, stop_code, stop_name, stop_desc, stop_lat, stop_lon,
            zone_id, stop_url, location_type, parent_station, platform_code
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)
        "#,
        stop.stop_id,
        stop.stop_code,
        stop.stop_name,
        stop.stop_desc,
        stop.stop_lat,
        stop.stop_lon,
        stop.zone_id,
        stop.stop_url,
        stop.location_type,
        stop.parent_station,
        stop.platform_code
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_route(route: &Route, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO routes (
            route_id, route_short_name, route_long_name, route_desc, route_type,
            route_url, route_color, route_text_color
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        "#,
        route.route_id,
        route.route_short_name,
        route.route_long_name,
        route.route_desc,
        route.route_type,
        route.route_url,
        route.route_color,
        route.route_text_color
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_trip(trip: &Trip, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO trips (
            route_id, service_id, trip_id, trip_headsign,
            direction_id, block_id, shape_id
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7)
        "#,
        trip.route_id,
        trip.service_id,
        trip.trip_id,
        trip.trip_headsign,
        trip.direction_id,
        trip.block_id,
        trip.shape_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_stop_time(stop_time: &StopTime, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO stop_times (
            trip_id, arrival_time, departure_time, stop_id,
            stop_sequence, pickup_type, drop_off_type
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7)
        "#,
        stop_time.trip_id,
        stop_time.arrival_time,
        stop_time.departure_time,
        stop_time.stop_id,
        stop_time.stop_sequence,
        stop_time.pickup_type,
        stop_time.drop_off_type
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_calendar(calendar: &Calendar, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO calendar (
            service_id, monday, tuesday, wednesday, thursday,
            friday, saturday, sunday, start_date, end_date
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10)
        "#,
        calendar.service_id,
        calendar.monday,
        calendar.tuesday,
        calendar.wednesday,
        calendar.thursday,
        calendar.friday,
        calendar.saturday,
        calendar.sunday,
        calendar.start_date as i64,
        calendar.end_date as i64
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_calendar_date(cd: &CalendarDate, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO calendar_dates (
            service_id, date, exception_type
        )
        VALUES ($1,$2,$3)
        "#,
        cd.service_id,
        cd.date as i64,
        cd.exception_type
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_shape(shape: &Shape, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO shapes (
            shape_id, shape_pt_lat, shape_pt_lon, shape_pt_sequence
        )
        VALUES ($1,$2,$3,$4)
        "#,
        shape.shape_id,
        shape.shape_pt_lat,
        shape.shape_pt_lon,
        shape.shape_pt_sequence
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_feed_info(feed: &FeedInfo, pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO feed_info (
            feed_publisher_name, feed_publisher_url,
            feed_lang, feed_start_date, feed_end_date
        )
        VALUES ($1,$2,$3,$4,$5)
        "#,
        feed.feed_publisher_name,
        feed.feed_publisher_url,
        feed.feed_lang,
        feed.feed_start_date,
        feed.feed_end_date
    )
    .execute(pool)
    .await?;
    Ok(())
}
