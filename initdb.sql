CREATE TABLE IF NOT EXISTS networks (
    id TEXT PRIMARY KEY,
    network TEXT NOT NULL,
    available INTEGER NOT NULL,
    used INTEGER NOT NULL,
    free INTEGER NOT NULL,
    vlan INTEGER,
    description TEXT,
    father TEXT,
    FOREIGN KEY (father) REFERENCES networks(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS offices (
    id TEXT PRIMARY KEY, 
    rack TEXT,
    address TEXT UNIQUE
);

CREATE TABLE IF NOT EXISTS devices (
    ip TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    location TEXT,
    network_id TEXT NOT NULL,
    credential BLOB,
    PRIMARY KEY (ip, network_id),
    FOREIGN KEY (network_id) REFERENCES networks(id) ON DELETE CASCADE,
    FOREIGN KEY (location) REFERENCES location(id) ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS location {
    id TEXT,
    type text CHECK(type IN ("Rack", "Desk", "Rack Cabinet"))
    office_id TEXT,
    FOREIGN KEY (office_id) REFERENCES offices(id) ON DELETE ON CASCADE,
    PRIMARY KEY (id)
}

CREATE TABLE IF NOT EXISTS service {
    port INTEGER,
    ip TEXT,
    network_id TEXT,
    service_id TEXT NOT NULL,
    description TEXT,
    type TEXT CHECK (type IN ('Container','Local')),
    PRIMARY KEY (port, ip, network_id),
    FOREIGN KEY (service_id) REFERENCES service (id) ON DELETE NO ACTION,
    FOREIGN KEY (ip, network_id) REFERENCES devices (ip, network_id) ON DELETE CASCADE
}

CREATE TABLE IF NO EXISTS services {
    id TEXT,
    name TEXT NOT NULL,
    version TEXT,
    PRIMARY KEY (id)
}

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE,
    password TEXT,
    role TEXT CHECK (role IN ('Admin', 'Operator', 'Guest')) 
);