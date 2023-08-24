PRAGMA foreign_keys = ON;

CREATE TABLE location_types
(
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL COLLATE NOCASE UNIQUE,
    description TEXT NOT NULL DEFAULT ''
);

INSERT INTO location_types (name, description)
VALUES ('Staging', 'Staging area for cars originating from or terminating at locations off the layout.'),
       ('Yard', 'A yard physically present on the layout for sorting cars into trains.'),
       ('Industry', 'A type of customer that takes delivery of cars.');

CREATE TABLE locations
(
    id               INTEGER PRIMARY KEY,
    name             TEXT COLLATE NOCASE UNIQUE             NOT NULL,
    description      TEXT                                   NOT NULL DEFAULT '',
    location_type_id INTEGER REFERENCES location_types (id) NOT NULL
);

CREATE TABLE waybills
(
    id               INTEGER PRIMARY KEY,
    consignee        TEXT                              NOT NULL,
    description      TEXT                              NOT NULL DEFAULT '',
    routing          TEXT                              NOT NULL DEFAULT '',
    via              TEXT                              NOT NULL DEFAULT '',
    shipper          TEXT                              NOT NULL DEFAULT '',
    from_location_id INTEGER REFERENCES locations (id) NOT NULL,
    to_location_id   INTEGER REFERENCES locations (id) NOT NULL,
    next_waybill_id  INTEGER REFERENCES waybills (id)
);

CREATE TABLE trains
(
    id   INTEGER PRIMARY KEY,
    name TEXT COLLATE NOCASE UNIQUE NOT NULL
);

CREATE TABLE car_types
(
    id          INTEGER PRIMARY KEY,
    name        TEXT COLLATE NOCASE UNIQUE NOT NULL,
    description TEXT                       NOT NULL DEFAULT ''
);

CREATE TABLE roads
(
    id        INTEGER PRIMARY KEY,
    name      TEXT COLLATE NOCASE UNIQUE NOT NULL,
    shorthand TEXT                       NOT NULL DEFAULT ''
);

CREATE TABLE car_cards
(
    id          INTEGER PRIMARY KEY,
    number      TEXT                              NOT NULL,
    car_type_id INTEGER REFERENCES car_types (id) NOT NULL,
    is_loaded   BOOLEAN                           NOT NULL,
    notes       TEXT                              NOT NULL DEFAULT '',
    waybill_id  INTEGER REFERENCES waybills (id),
    location_id INTEGER REFERENCES locations (id),
    road_id     INTEGER REFERENCES roads (id)     NOT NULL,
    train_id    INTEGER REFERENCES trains (id)
);
