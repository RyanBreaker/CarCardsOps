CREATE TABLE location_types
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT         NOT NULL DEFAULT ''
);

INSERT INTO location_types (name, description)
VALUES ('Staging', 'Staging area for cars originating from or terminating at locations off the layout.'),
       ('Yard', 'A yard physically present on the layout for sorting cars into trains.');

CREATE TABLE locations
(
    id               SERIAL PRIMARY KEY,
    name             VARCHAR(255)                           NOT NULL,
    description      TEXT                                   NOT NULL DEFAULT '',
    location_type_id INTEGER REFERENCES location_types (id) NOT NULL
);

CREATE TABLE waybills
(
    id               SERIAL PRIMARY KEY,
    name             TEXT                              NOT NULL,
    description      TEXT                              NOT NULL DEFAULT '',
    routing          TEXT                              NOT NULL DEFAULT '',
    from_location_id INTEGER REFERENCES locations (id) NOT NULL,
    to_location_id   INTEGER REFERENCES locations (id) NOT NULL
);

CREATE TABLE trains
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL DEFAULT ''
);

CREATE TABLE car_types
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(255) NOT NULL,
    description TEXT         NOT NULL DEFAULT ''
);

CREATE TABLE roads
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE car_cards
(
    id          SERIAL PRIMARY KEY,
    number      VARCHAR(255)                      NOT NULL,
    car_type_id INTEGER REFERENCES car_types (id) NOT NULL,
    is_loaded   BOOLEAN                           NOT NULL,
    notes       TEXT                              NOT NULL DEFAULT '',
    waybill_id  INTEGER REFERENCES waybills (id),
    location_id INTEGER REFERENCES locations (id),
    road_id     INTEGER REFERENCES roads (id)     NOT NULL,
    train_id    INTEGER REFERENCES trains (id)
);
