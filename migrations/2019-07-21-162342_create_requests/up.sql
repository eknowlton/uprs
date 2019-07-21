CREATE TABLE requests (
        id INTEGER NOT NULL PRIMARY KEY,
        name VARCHAR NOT NULL,
        uri VARCHAR NOT NULL,
        response_status_code VARCHAR NOT NULL,
        success BOOLEAN NOT NULL DEFAULT 1
);
