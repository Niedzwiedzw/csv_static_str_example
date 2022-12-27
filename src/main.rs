use eyre::{
    Result,
    WrapErr,
};
use serde::{
    Deserialize,
    Serialize,
};
use std::borrow::Cow;

pub static RAW_DATA: &str = include_str!("../some-static-file.csv"); // this is 500MB in size

// ~300 garbage fields
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RawEntry {
    #[serde(rename = "#", borrow)]
    pub index: Cow<'static, str>,
    #[serde(rename = "id", borrow)]
    pub id: Cow<'static, str>,
    #[serde(rename = "idZaokragleniaCen", borrow)]
    pub idZaokragleniaCen: Cow<'static, str>,
}

impl RawEntry {
    pub fn entries() -> impl Iterator<Item = Result<Self>> {
        let mut reader = csv::Reader::from_reader(&mut std::io::Cursor::new(RAW_DATA));
        reader
            .into_deserialize::<RawEntry>()
            .into_iter()
            .enumerate()
            .map(|(idx, res)| res.wrap_err_with(|| format!("bad entry at index {idx}")))
    }
}

fn main() {
    RawEntry::entries().for_each(|e| println!("{e:?}"));
}
