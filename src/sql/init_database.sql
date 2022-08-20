CREATE TABLE users (
    id TEXT PRIMARY KEY,
    email TEXT,
    fullname TEXT,
    password TEXT,
    number TEXT
);

CREATE TABLE events (
    id TEXT PRIMARY KEY,
    name TEXT,
    time INTEGER,
    address1 TEXT,
    address2 TEXt,
    city TEXT,
    state TEXT,
    zipcode TEXT,
    creator_id TEXT,
    FOREIGN KEY (creator_id) REFERENCES users (id)
);

CREATE TABLE rides (
    rider_id TEXT,
    driver_id TEXT,
    event_id TEXT,
    pickup_location TEXT,
    campus TEXT,
    FOREIGN KEY (rider_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE
);

CREATE TABLE vehicles (
    id TEXT PRIMARY KEY,
    user_id TEXT,
    color TEXT,
    make TEXT,
    model TEXT,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TABLE drivers (
    event_id TEXT,
    driver_id TEXT,
    seats INTEGER,
    vehicle_id TEXT,
    campus TEXT,
    FOREIGN KEY (event_id) REFERENCES events (id) ON DELETE CASCADE,
    FOREIGN KEY (driver_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (vehicle_id) REFERENCES vehicles (id) ON DELETE CASCADE
);

