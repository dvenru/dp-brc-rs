mod query;
use query::*;

use chrono::prelude::*;
use rusqlite::ffi::Error;
use rusqlite::{Connection, Result};

#[derive(Clone, Debug)]
pub struct BarCodeData {
    pub name: String,
    pub count: u32,
    pub storage_location: String,
    pub brcode: String,
}

#[derive(Clone, Debug)]
pub struct BarCodeHistoryData {
    pub name: String,
    pub count: u32,
    pub storage_location: String,
    pub brcode: String,
    pub time_change: String
}

pub struct DataBase {
    connection: Connection
}

impl DataBase {
    pub fn new(connecting_string: &str) -> Result<Self, Error> {
        Ok(DataBase {
            connection: Connection::open(connecting_string).unwrap()
        })
    }

    pub fn init(&self) -> Result<(), Error> {
        self.connection.execute(
            CREATE_BARCODE_TABLE,
            ()
        ).unwrap();

        self.connection.execute(
            CREATE_HISTORY_TABLE,
            ()
        ).unwrap();

        Ok(())
    }

    pub fn append(&mut self, data: BarCodeData) -> Result<()> {
        let trx = self.connection.transaction().unwrap();

        let _ = trx.execute(
            APPEND_BARCODE,
            (&data.name, &data.count, &data.storage_location, &data.brcode)
        );

        let now = Utc::now();
        let date = format!(
            "{}/{}/{} || {}:{}",
            now.day(),
            now.month(),
            now.year(),
            now.hour(),
            now.minute()
        );

        let _ = trx.execute(
            APPEND_HISTORY,
            (&data.name, &data.count, &data.storage_location, &data.brcode, &date)
        );

        trx.commit()
    }

    pub fn update(&mut self, data: BarCodeData) -> Result<()> {
        let trx = self.connection.transaction().unwrap();

        let _ = trx.execute(
            UPDATE_BARCODE,
            (&data.name, &data.count, &data.storage_location, &data.brcode)
        );

        let now = Utc::now();
        let date = format!(
            "{}/{}/{} || {}:{}",
            now.day(),
            now.month(),
            now.year(),
            now.hour(),
            now.minute()
        );

        let _ = trx.execute(
            APPEND_HISTORY,
            (&data.name, &data.count, &data.storage_location, &data.brcode, &date)
        );

        trx.commit()
    }

    pub fn get_all(&self) -> Result<Vec<BarCodeData>, Error> {
        let mut stmt = self.connection.prepare(GETALL_BARCODE).unwrap();
        let query_res = stmt.query_map((), |row| {
            Ok(BarCodeData {
                name: row.get(0).unwrap(),
                count: row.get(1).unwrap(),
                storage_location: row.get(2).unwrap(),
                brcode: row.get(3).unwrap()
            })
        }).unwrap();

        let mut res = Vec::new();
        for bar in query_res {
            res.push(bar.unwrap())
        }

        Ok(res)
    }

    pub fn get_history(&self, data: BarCodeData) -> Result<Vec<BarCodeHistoryData>, Error> {
        let mut stmt = self.connection.prepare(GET_HISTORY).unwrap();
        let query_res = stmt.query_map([&data.brcode], |row| {
            Ok(BarCodeHistoryData {
                name: row.get(0).unwrap(),
                count: row.get(1).unwrap(),
                storage_location: row.get(2).unwrap(),
                brcode: row.get(3).unwrap(),
                time_change: row.get(4).unwrap()
            })
        }).unwrap();

        let mut res = Vec::new();
        for his in query_res {
            res.push(his.unwrap())
        }

        Ok(res)
    }
}