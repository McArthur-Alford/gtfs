//! DB Tests
//!
//! Specifically tests db queries and operations.
//! Does not test a gtfs dataset properly.

#[cfg(test)]
use super::queries::{
    insert_agency, insert_calendar, insert_calendar_date, insert_feed_info, insert_route,
    insert_shape, insert_stop, insert_stop_time, insert_trip,
};
use super::types::*;
use chrono::TimeDelta;
use sqlx::PgPool;
use tracing_test::traced_test;

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_agency(pool: PgPool) -> sqlx::Result<()> {
    let agency = Agency {
        agency_name: "Translink".into(),
        agency_url: "https://translink.com.au/".into(),
        agency_timezone: "Australia/Brisbane".into(),
        agency_lang: Some("en".into()),
        agency_phone: Some("13 12 30".into()),
    };
    insert_agency(&agency, &pool).await?;

    let row = sqlx::query_as!(
        Agency,
        "SELECT * FROM agency WHERE agency_name = $1",
        &agency.agency_name
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, agency);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_calendar(pool: PgPool) -> sqlx::Result<()> {
    let cal = Calendar {
        service_id: "GCLR 24_25-36991".into(),
        monday: true,
        tuesday: true,
        wednesday: true,
        thursday: true,
        friday: false,
        saturday: false,
        sunday: false,
        start_date: 20250617,
        end_date: 20250626,
    };
    insert_calendar(&cal, &pool).await?;

    let row = sqlx::query_as!(
        Calendar,
        "SELECT * FROM calendar WHERE service_id = $1",
        &cal.service_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, cal);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_calendar_date(pool: PgPool) -> sqlx::Result<()> {
    let cd = CalendarDate {
        service_id: "BCC 25_26-39839".into(),
        date: 20250813,
        exception_type: 1,
    };
    insert_calendar_date(&cd, &pool).await?;

    let row = sqlx::query_as!(
        CalendarDate,
        "SELECT * FROM calendar_dates WHERE service_id = $1 AND date = $2",
        &cd.service_id,
        cd.date as i32
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, cd);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_feed_info(pool: PgPool) -> sqlx::Result<()> {
    let feed = FeedInfo {
        feed_publisher_name: "Department of Transport and Main Roads - Translink Division".into(),
        feed_publisher_url: "https://www.translink.com.au/".into(),
        feed_lang: Some("en".into()),
        feed_start_date: Some(20250617),
        feed_end_date: Some(20250816),
    };
    insert_feed_info(&feed, &pool).await?;

    let row = sqlx::query_as!(
        FeedInfo,
        "SELECT * FROM feed_info WHERE feed_publisher_name = $1",
        &feed.feed_publisher_name
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, feed);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_route(pool: PgPool) -> sqlx::Result<()> {
    let route = Route {
        route_id: "19-4158".into(),
        route_short_name: Some("19".into()),
        route_long_name: Some("Salisbury - PA Hospital StationLink".into()),
        route_desc: None,
        route_type: 3,
        route_url: Some("https://jp.translink.com.au/plan-your-journey/timetables/bus/T/19".into()),
        route_color: Some("E463A4".into()),
        route_text_color: Some("000000".into()),
    };
    insert_route(&route, &pool).await?;

    let row = sqlx::query_as!(
        Route,
        "SELECT * FROM routes WHERE route_id = $1",
        &route.route_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, route);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_shape(pool: PgPool) -> sqlx::Result<()> {
    let shape = Shape {
        shape_id: "190008".into(),
        shape_pt_lat: -27.553364,
        shape_pt_lon: 153.023933,
        shape_pt_sequence: 10001,
    };
    insert_shape(&shape, &pool).await?;

    let row = sqlx::query_as!(
        Shape,
        "SELECT * FROM shapes WHERE shape_id = $1 AND shape_pt_sequence = $2",
        &shape.shape_id,
        &shape.shape_pt_sequence
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, shape);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_stop(pool: PgPool) -> sqlx::Result<()> {
    let stop = Stop {
        stop_id: "1".into(),
        stop_code: Some("000001".into()),
        stop_name: Some("Herschel Street Stop 1 near North Quay".into()),
        stop_desc: None,
        stop_lat: Some(-27.467834),
        stop_lon: Some(153.019079),
        zone_id: Some("1".into()),
        stop_url: Some("https://translink.com.au/stop/000001/gtfs/".into()),
        location_type: Some(0),
        parent_station: None,
        platform_code: None,
    };
    insert_stop(&stop, &pool).await?;

    let row = sqlx::query_as!(
        Stop,
        "SELECT * FROM stops WHERE stop_id = $1",
        &stop.stop_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, stop);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_trip(pool: PgPool) -> sqlx::Result<()> {
    let route = Route {
        route_id: "R600-3454".into(),
        route_short_name: Some("19".into()),
        route_long_name: Some("Salisbury - PA Hospital StationLink".into()),
        route_desc: None,
        route_type: 3,
        route_url: Some("https://jp.translink.com.au/plan-your-journey/timetables/bus/T/19".into()),
        route_color: Some("E463A4".into()),
        route_text_color: Some("000000".into()),
    };
    insert_route(&route, &pool).await?;

    let trip = Trip {
        route_id: "R600-3454".into(),
        service_id: "ATS_KBL 25-38992".into(),
        trip_id: "32324843-ATS_KBL 25-38992".into(),
        trip_headsign: Some("Bowen Hills station".into()),
        direction_id: Some(false),
        block_id: None,
        shape_id: Some("R6000053".into()),
    };
    insert_trip(&trip, &pool).await?;

    let row = sqlx::query_as!(
        Trip,
        "SELECT * FROM trips WHERE trip_id = $1",
        &trip.trip_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, trip);
    Ok(())
}

#[traced_test]
#[sqlx::test(migrator = "super::MIGRATOR")]
async fn test_stop_time(pool: PgPool) -> sqlx::Result<()> {
    let stop = Stop {
        stop_id: "1".into(),
        stop_code: Some("000001".into()),
        stop_name: Some("Herschel Street Stop 1 near North Quay".into()),
        stop_desc: None,
        stop_lat: Some(-27.467834),
        stop_lon: Some(153.019079),
        zone_id: Some("1".into()),
        stop_url: Some("https://translink.com.au/stop/000001/gtfs/".into()),
        location_type: Some(0),
        parent_station: None,
        platform_code: None,
    };
    insert_stop(&stop, &pool).await?;

    let route = Route {
        route_id: "R600-3454".into(),
        route_short_name: Some("19".into()),
        route_long_name: Some("Salisbury - PA Hospital StationLink".into()),
        route_desc: None,
        route_type: 3,
        route_url: Some("https://jp.translink.com.au/plan-your-journey/timetables/bus/T/19".into()),
        route_color: Some("E463A4".into()),
        route_text_color: Some("000000".into()),
    };
    insert_route(&route, &pool).await?;

    let trip = Trip {
        route_id: "R600-3454".into(),
        service_id: "ATS_KBL 25-38992".into(),
        trip_id: "32324843-ATS_KBL 25-38992".into(),
        trip_headsign: Some("Bowen Hills station".into()),
        direction_id: Some(false),
        block_id: None,
        shape_id: Some("R6000053".into()),
    };
    insert_trip(&trip, &pool).await?;

    let stop_time = StopTime {
        trip_id: trip.trip_id.clone(),
        arrival_time: Some(
            TimeDelta::try_minutes(16 * 60 + 50)
                .unwrap()
                .try_into()
                .unwrap(),
        ),
        departure_time: TimeDelta::try_minutes(16 * 60 + 50)
            .unwrap()
            .try_into()
            .unwrap(),
        stop_id: stop.stop_id.clone(),
        stop_sequence: 1,
        pickup_type: 0,
        drop_off_type: 0,
    };
    insert_stop_time(&stop_time, &pool).await?;

    let row: StopTime = sqlx::query_as!(
        StopTime,
        "SELECT * FROM stop_times WHERE trip_id = $1 AND stop_id = $2",
        &stop_time.trip_id,
        &stop_time.stop_id
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row, stop_time);
    Ok(())
}
