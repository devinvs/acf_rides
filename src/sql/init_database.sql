CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email TEXT,
    fullname TEXT,
    password TEXT,
    number TEXT,
    campus TEXT
);

CREATE TABLE events (
    id INTEGER PRIMARY KEY,
    name TEXT,
    time INTEGER,
    address1 TEXT,
    address2 TEXt,
    city TEXT,
    state TEXT,
    zipcode TEXT
);

CREATE TABLE rides (
    rider_id INTEGER,
    driver_id INTEGER,
    event_id INTEGER,
    pickup_location TEXT,
    FOREIGN KEY (rider_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE
);

CREATE TABLE vehicles (
    id INTEGER PRIMARY KEY,
    user_id INTEGER,
    color TEXT,
    make TEXT,
    model TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE drivers (
    event_id INTEGER,
    driver_id INTEGER,
    seats INTEGER,
    vehicle_id INTEGER,
    FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (vehicle_id) REFERENCES vehicles (id) ON DELETE CASCADE
);

