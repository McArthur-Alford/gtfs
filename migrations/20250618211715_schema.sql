-- Auto generated schema using the wonderful script from:
-- https://github.com/tyleragreen/gtfs-schema/tree/master
 
DROP TABLE IF EXISTS agency CASCADE;
CREATE TABLE agency
(
  agency_name            text NOT NULL,
  agency_url             text NOT NULL,
  agency_timezone        text NOT NULL,
  agency_lang            text NULL,
  agency_phone           text NULL
);

DROP TABLE IF EXISTS stops CASCADE;
CREATE TABLE stops
(
  stop_id                text PRIMARY KEY,
  stop_code              text NULL,
  stop_name              text NULL CHECK (location_type >= 0 AND location_type <= 2 AND stop_name IS NOT NULL OR location_type > 2),
  stop_desc              text NULL,
  stop_lat               double precision NULL CHECK (location_type >= 0 AND location_type <= 2 AND stop_name IS NOT NULL OR location_type > 2),
  stop_lon               double precision NULL CHECK (location_type >= 0 AND location_type <= 2 AND stop_name IS NOT NULL OR location_type > 2),
  zone_id                text NULL,
  stop_url               text NULL,
  location_type          integer NULL CHECK (location_type >= 0 AND location_type <= 4),
  parent_station         text NULL CHECK (location_type IS NULL OR location_type = 0 OR location_type = 1 AND parent_station IS NULL OR location_type >= 2 AND location_type <= 4 AND parent_station IS NOT NULL),
  platform_code          text NULL
);

DROP TABLE IF EXISTS routes CASCADE;
CREATE TABLE routes
(
  route_id               text PRIMARY KEY,
  route_short_name       text NULL,
  route_long_name        text NULL CHECK (route_short_name IS NOT NULL OR route_long_name IS NOT NULL),
  route_desc             text NULL,
  route_type             integer NOT NULL,
  route_url              text NULL,
  route_color            text NULL CHECK (route_color ~ $$[a-fA-F0-9]{6}$$ OR route_color = ''),
  route_text_color       text NULL CHECK (route_color ~ $$[a-fA-F0-9]{6}$$ OR route_color = '')
);

DROP TABLE IF EXISTS trips CASCADE;
CREATE TABLE trips
(
  route_id               text NOT NULL REFERENCES routes ON DELETE CASCADE ON UPDATE CASCADE,
  service_id             text NOT NULL,
  trip_id                text NOT NULL PRIMARY KEY,
  trip_headsign          text NULL,
  direction_id           boolean NULL,
  block_id               text NULL,
  shape_id               text NULL
);

DROP TABLE IF EXISTS stop_times CASCADE;
CREATE TABLE stop_times
(
  trip_id                text NOT NULL REFERENCES trips ON DELETE CASCADE ON UPDATE CASCADE,
  arrival_time           interval NULL,
  departure_time         interval NOT NULL,
  stop_id                text NOT NULL REFERENCES stops ON DELETE CASCADE ON UPDATE CASCADE,
  stop_sequence          integer NOT NULL CHECK (stop_sequence >= 0),
  pickup_type            integer NOT NULL CHECK (pickup_type >= 0 AND pickup_type <= 3),
  drop_off_type          integer NOT NULL CHECK (drop_off_type >= 0 AND drop_off_type <= 3)
);

DROP TABLE IF EXISTS calendar CASCADE;
CREATE TABLE calendar
(
  service_id             text PRIMARY KEY,
  monday                 boolean NOT NULL,
  tuesday                boolean NOT NULL,
  wednesday              boolean NOT NULL,
  thursday               boolean NOT NULL,
  friday                 boolean NOT NULL,
  saturday               boolean NOT NULL,
  sunday                 boolean NOT NULL,
  start_date             integer NOT NULL,
  end_date               integer NOT NULL
);

DROP TABLE IF EXISTS calendar_dates CASCADE;
CREATE TABLE calendar_dates
(
  service_id             text NOT NULL,
  date                   integer NOT NULL,
  exception_type         integer NOT NULL CHECK (exception_type >= 1 AND exception_type <= 2)
);

DROP TABLE IF EXISTS shapes CASCADE;
CREATE TABLE shapes
(
  shape_id               text NOT NULL,
  shape_pt_lat           double precision NOT NULL,
  shape_pt_lon           double precision NOT NULL,
  shape_pt_sequence      integer NOT NULL CHECK (shape_pt_sequence >= 0)
);

DROP TABLE IF EXISTS feed_info CASCADE;
CREATE TABLE feed_info
(
  feed_publisher_name    text NOT NULL,
  feed_publisher_url     text NOT NULL,
  feed_lang              text NULL,
  feed_start_date        integer NULL,
  feed_end_date          integer NULL
);

-- \COPY agency FROM './seq_gtfs/agency.txt' (FORMAT CSV, HEADER)
-- \COPY stops FROM './seq_gtfs/stops.txt' (FORMAT CSV, HEADER)
-- \COPY routes FROM './seq_gtfs/routes.txt' (FORMAT CSV, HEADER)
-- \COPY trips FROM './seq_gtfs/trips.txt' (FORMAT CSV, HEADER)
-- \COPY stop_times FROM './seq_gtfs/stop_times.txt' (FORMAT CSV, HEADER)
-- \COPY calendar FROM './seq_gtfs/calendar.txt' (FORMAT CSV, HEADER)
-- \COPY calendar_dates FROM './seq_gtfs/calendar_dates.txt' (FORMAT CSV, HEADER)
-- \COPY shapes FROM './seq_gtfs/shapes.txt' (FORMAT CSV, HEADER)
-- \COPY feed_info FROM './seq_gtfs/feed_info.txt' (FORMAT CSV, HEADER)
