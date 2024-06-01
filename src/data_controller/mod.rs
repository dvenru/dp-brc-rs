mod query;
use query::*;

use chrono::prelude::*;
use rusqlite::ffi::Error;
use rusqlite::{Connection, Result, Transaction};

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

pub struct DataBase {}

impl DataBase {
    pub fn new() -> Self {
        DataBase { }
    }

    pub fn init(&self) -> Result<(), Error> {
        let conn = Connection::open(CONNECTING_STRING).unwrap();

        conn.execute(
            CREATE_BARCODE_TABLE,
            ()
        ).unwrap();

        conn.execute(
            CREATE_HISTORY_TABLE,
            ()
        ).unwrap();

        Ok(())
    }

    pub fn append(&mut self, data: BarCodeData) -> Result<()> {
        let mut conn = Connection::open(CONNECTING_STRING).unwrap();
        let trx = conn.transaction().unwrap();

        let _ = trx.execute(
            APPEND_BARCODE,
            (&data.name, &data.count, &data.storage_location, &data.brcode)
        );

        self.update_history(&trx, data);

        trx.commit()
    }

    pub fn update(&mut self, data: BarCodeData) -> Result<()> {
        let mut conn = Connection::open(CONNECTING_STRING).unwrap();
        let trx = conn.transaction().unwrap();

        let _ = trx.execute(
            UPDATE_BARCODE,
            (&data.name, &data.count, &data.storage_location, &data.brcode)
        );

        self.update_history(&trx, data);

        trx.commit()
    }

    fn update_history(&self, trx: &Transaction, data: BarCodeData) {
        let now = Local::now();
        let date = now.format("%d/%m/%Y || %H:%M").to_string();

        let last_id = self.get_barcode_id(trx, &data.brcode);
        
        let _ = trx.execute(
            APPEND_HISTORY,
            (&data.name, &data.count, &data.storage_location, &data.brcode, &date, &last_id)
        );
    }

    fn get_barcode_id(&self, trx: &Transaction, barcode: &String) -> i64 {
        let mut stmt = trx.prepare(GET_BARCODE_ID).unwrap();

        let query_res = stmt.query_map([barcode], |row| {
            Ok(row.get(0).unwrap())
        }).unwrap()
            .map(|id| id.unwrap())
            .collect::<Vec<i64>>();

        query_res[0]
    }

    pub fn name_is_unique(&self, name: String) -> bool {
        let conn = Connection::open(CONNECTING_STRING).unwrap();
        let mut stmt = conn.prepare(GET_BARCODE_NAME).unwrap();

        let name = name.clone()
            .trim()
            .to_lowercase();

        let query_res = stmt.query_map((), |row| {
            Ok(row.get::<usize, String>(0).unwrap())
        }).unwrap()
            .map(|n| n.unwrap().trim().to_lowercase())
            .collect::<Vec<String>>();
        
        for n in query_res {
            if n == name {
                return false;
            }
        }

        true
    }

    pub fn get_items(&self) -> Result<Vec<BarCodeData>, Error> {
        let conn = Connection::open(CONNECTING_STRING).unwrap();
        let mut stmt = conn.prepare(GET_BARCODE_ALL).unwrap();

        let query_res = stmt.query_map((), |row| {
            Ok(BarCodeData {
                name: row.get(0).unwrap(),
                count: row.get(1).unwrap(),
                storage_location: row.get(2).unwrap(),
                brcode: row.get(3).unwrap()
            })
        }).unwrap()
            .map(|bar| bar.unwrap())
            .collect::<Vec<BarCodeData>>();

        Ok(query_res)
    }

    pub fn get_history(&self, data: Option<&BarCodeData>) -> Result<Vec<BarCodeHistoryData>, Error> {
        let conn = Connection::open(CONNECTING_STRING).unwrap();

        match data {
            Some(data) => {
                let mut stmt = conn.prepare(GET_HISTORY_BY_BARCODE).unwrap();

                let query_res = stmt.query_map([&data.brcode], |row| {
                    Ok(BarCodeHistoryData {
                        name: row.get(0).unwrap(),
                        count: row.get(1).unwrap(),
                        storage_location: row.get(2).unwrap(),
                        brcode: row.get(3).unwrap(),
                        time_change: row.get(4).unwrap()
                    })
                }).unwrap()
                    .map(|his| his.unwrap())
                    .collect::<Vec<BarCodeHistoryData>>();

                return Ok(query_res);
            }
            None => {
                let mut stmt = conn.prepare(GET_HISTORY_ALL).unwrap();
                let query_res = stmt.query_map((), |row| {
                    Ok(BarCodeHistoryData {
                        name: row.get(0).unwrap(),
                        count: row.get(1).unwrap(),
                        storage_location: row.get(2).unwrap(),
                        brcode: row.get(3).unwrap(),
                        time_change: row.get(4).unwrap()
                    })
                }).unwrap()
                    .map(|his| his.unwrap())
                    .collect::<Vec<BarCodeHistoryData>>();

                return Ok(query_res);
            }
        }
    }
}