//oid,country_or_area,year,description,magnitude,value,category
use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};
use std::io;
use tree_buf::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Decode, Encode)]
pub struct Record {
    oid: String,
    country_or_area: String,
    year: u16,
    description: String,
    magnitude: String,
    value: String,
    category: String,
}

pub fn decode(bytes: &[u8]) -> Vec<Record> {
    let reader = io::Cursor::new(&bytes[..]);
    let mut rdr = Reader::from_reader(reader);
    rdr.deserialize().into_iter().map(|r| r.unwrap()).collect()
}

pub fn encode(records: &[Record]) -> Vec<u8> {
    let mut writer = Writer::from_writer(vec![]);
    for record in records.iter() {
        writer.serialize(record).unwrap();
    }

    writer.into_inner().unwrap()
}
