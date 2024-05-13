pub const CREATE_BARCODE_TABLE: &str = "CREATE TABLE IF NOT EXISTS barcodes (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    count INTEGER NOT NULL,
    storage_location TEXT NOT NULL,
    brcode TEXT NOT NULL UNIQUE
)";

pub const CREATE_HISTORY_TABLE: &str = "CREATE TABLE IF NOT EXISTS barcodes_history (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    count INTEGER NOT NULL,
    storage_location TEXT NOT NULL,
    brcode TEXT NOT NULL REFERENCES barcodes(brcode),
    time_change TEXT NOT NULL
)";

pub const APPEND_BARCODE: &str = "INSERT INTO barcodes (name, count, storage_location, brcode) VALUES (?1, ?2, ?3, ?4)";

pub const APPEND_HISTORY: &str = "INSERT INTO barcodes_history (name, count, storage_location, brcode, time_change) VALUES (?1, ?2, ?3, ?4, ?5)";

// pub const DELETE_BARCODE: &str = "DELETE FROM barcodes WHERE brcode=?";

pub const GETALL_BARCODE: &str = "SELECT name, count, storage_location, brcode FROM barcodes";

pub const GET_HISTORY: &str = "SELECT name, count, storage_location, brcode, time_change FROM barcodes_history WHERE brcode=?";

pub const UPDATE_BARCODE: &str = "UPDATE barcodes SET name=?1, count=?2, storage_location=?3 WHERE brcode=?4";