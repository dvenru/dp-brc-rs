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
    brcode TEXT NOT NULL,
    time_change TEXT NOT NULL,
    origin_id INTEGER,
    FOREIGN KEY (origin_id) REFERENCES barcodes(id)
)";

pub const APPEND_BARCODE: &str = "INSERT INTO barcodes (name, count, storage_location, brcode) VALUES (?1, ?2, ?3, ?4)";

pub const APPEND_HISTORY: &str = "INSERT INTO barcodes_history (name, count, storage_location, brcode, time_change, origin_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

pub const GETALL_BARCODE: &str = "SELECT name, count, storage_location, brcode FROM barcodes";

pub const GET_HISTORY_ALL: &str = "SELECT name, count, storage_location, brcode, time_change FROM barcodes_history";

pub const GET_HISTORY_BY_BARCODE: &str = "SELECT name, count, storage_location, brcode, time_change FROM barcodes_history WHERE brcode=?";

pub const UPDATE_BARCODE: &str = "UPDATE barcodes SET name=?1, count=?2, storage_location=?3 WHERE brcode=?4";
