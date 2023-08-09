PRAGMA foreign_keys = ON;

CREATE TABLE location_types
(
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT ''
);

INSERT INTO location_types (name, description)
VALUES ('Staging', 'Staging area for cars originating from or terminating at locations off the layout.'),
       ('Yard', 'A yard physically present on the layout for sorting cars into trains.');

CREATE TABLE locations
(
    id               INTEGER PRIMARY KEY,
    name             TEXT                                   NOT NULL,
    description      TEXT                                   NOT NULL DEFAULT '',
    location_type_id INTEGER REFERENCES location_types (id) NOT NULL
);

CREATE TABLE waybills
(
    id               INTEGER PRIMARY KEY,
    name             TEXT                              NOT NULL,
    description      TEXT                              NOT NULL DEFAULT '',
    routing          TEXT                              NOT NULL DEFAULT '',
    from_location_id INTEGER REFERENCES locations (id) NOT NULL,
    to_location_id   INTEGER REFERENCES locations (id) NOT NULL
);

CREATE TABLE trains
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL DEFAULT ''
);

CREATE TABLE car_types
(
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT ''
);

CREATE TABLE roads
(
    id   INTEGER PRIMARY KEY,
    name TEXT NOT NULL
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
