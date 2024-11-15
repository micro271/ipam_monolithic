CREATE TABLE IF NOT EXISTS networks (
    id TEXT PRIMARY KEY,
    network TEXT NOT NULL,
    available BLOB NOT NULL,
    used INTEGER NOT NULL,
    vlan INTEGER,
    description TEXT
);

CREATE TABLE IF NOT EXISTS offices (
    id TEXT PRIMARY KEY, 
    description TEXT,
    address TEXT UNIQUE
);

CREATE TABLE IF NOT EXISTS devices (
    ip TEXT NOT NULL,
    description TEXT,
    office_id TEXT,
    rack TEXT,
    room TEXT,
    status TEXT NOT NULL,
    network_id TEXT NOT NULL,
    credential BLOB,
    PRIMARY KEY (ip, network_id),
    FOREIGN KEY (network_id) REFERENCES networks(id) ON DELETE CASCADE,
    FOREIGN KEY (office_id) REFERENCES offices(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    password TEXT,
    role TEXT CHECK(role IN ('Admin', 'Operator', 'Guest')) 
);