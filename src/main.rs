pub mod bridge;
pub mod db;
pub mod gtfs;
pub mod vars;

use anyhow::{Result, bail};
use prost::Message;
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, error, info};
use tracing_subscriber::{EnvFilter, field::MakeExt};

use crate::{
    bridge::ToDB,
    db::Db,
    gtfs::{last_modified, load_realtime_gtfs, load_static_gtfs},
    transit_realtime::FeedMessage,
    vars::{REALTIME_URL, STATIC_URL, realtime_urls},
};

pub mod transit_realtime {
    include!(concat!(env!("OUT_DIR"), "/transit_realtime.rs"));
}

pub struct State {
    db: Db,
    client: Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .map_fmt_fields(|f| f.debug_alt())
        .init();

    // Set up the DB connection pool
    let mut db = Db::connect().await?;
    db.run_migrations().await?;

    // Set up the reqwest client
    let client = Client::new();

    let state = State { db, client };

    let gtfs = load_static_gtfs("./seq_gtfs.zip".to_owned()).await?;

    gtfs.insert_db(state.db.clone()).await;

    tokio::time::sleep(Duration::from_secs(10)).await;

    // let gtfs = gtfs.0.to_db().await;

    // debug!(gtfs=?gtfs);

    panic!();

    loop {
        if let Err(e) = poll().await {
            error!(e=?e);
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn poll() -> Result<()> {
    let pb = reqwest::get("https://gtfsrt.api.translink.com.au/api/realtime/SEQ/TripUpdates")
        .await?
        .bytes()
        .await?;

    let message = transit_realtime::FeedMessage::decode(pb)?;

    // let time = message.header.timestamp() as i64;
    // let time = DateTime::from_timestamp(time, 0);

    // debug!(header=?message.header);
    // debug!(time=?time);

    for entity in message.entity {
        let Some(trip) = entity.trip_update.as_ref() else {
            continue;
        };

        debug!(trip=?trip);
    }

    info!("Polled");

    Ok(())
}
