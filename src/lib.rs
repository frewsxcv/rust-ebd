//! Rust library for reading the [eBird Basic Dataset (EBD)][ebd].
//!
//! # Examples
//!
//! ```rust
//! use std::io;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut ebd_reader = ebd::Reader::from_reader(io::stdin());
//!
//!     while ebd_reader.advance()? {
//!         let record = ebd_reader.read_record()?;
//!
//!         println!(
//!             "(lng: {}, lat: {})",
//!             record.longitude,
//!             record.latitude
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::{fmt, io, marker};

pub struct Reader<'a, R: io::Read> {
    csv_reader: csv::Reader<R>,
    csv_byte_record: csv::ByteRecord,
    phantom_data: marker::PhantomData<&'a ()>,
}

impl<'a, R: io::Read> Reader<'a, R> {
    pub fn from_reader(reader: R) -> Self {
        let csv_reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(reader);
        let csv_byte_record = csv::ByteRecord::new();

        Reader {
            csv_reader,
            csv_byte_record,
            phantom_data: marker::PhantomData,
        }
    }

    pub fn advance<'b>(&'b mut self) -> csv::Result<bool> {
        self.csv_reader.read_byte_record(&mut self.csv_byte_record)
    }

    pub fn read_record(&'a self) -> csv::Result<Record<'a>> {
        self.csv_byte_record.deserialize(None)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Record<'a> {
    #[serde(rename = "GLOBAL UNIQUE IDENTIFIER")]
    pub global_unique_identifier: &'a str,

    #[serde(rename = "LAST EDITED DATE")]
    pub last_edited_date: &'a str,

    #[serde(rename = "TAXONOMIC ORDER")]
    pub taxonomic_order: &'a str,

    #[serde(rename = "CATEGORY")]
    pub category: &'a str,

    #[serde(rename = "COMMON NAME")]
    pub common_name: &'a str,

    #[serde(rename = "SCIENTIFIC NAME")]
    pub scientific_name: &'a str,

    #[serde(rename = "SUBSPECIES COMMON NAME")]
    pub subspecies_common_name: &'a str,

    #[serde(rename = "SUBSPECIES SCIENTIFIC NAME")]
    pub subspecies_scientific_name: &'a str,

    #[serde(rename = "OBSERVATION COUNT")]
    pub observation_count: &'a str,

    #[serde(rename = "BREEDING BIRD ATLAS CODE")]
    pub breeding_bird_atlas_code: &'a str,

    #[serde(rename = "BREEDING BIRD ATLAS CATEGORY")]
    pub breeding_bird_atlas_category: &'a str,

    #[serde(rename = "AGE/SEX")]
    pub agesex: &'a str,

    #[serde(rename = "COUNTRY")]
    pub country: &'a str,

    #[serde(rename = "COUNTRY CODE")]
    pub country_code: &'a str,

    #[serde(rename = "STATE")]
    pub state: &'a str,

    #[serde(rename = "STATE CODE")]
    pub state_code: &'a str,

    #[serde(rename = "COUNTY")]
    pub county: &'a str,

    #[serde(rename = "COUNTY CODE")]
    pub county_code: &'a str,

    #[serde(rename = "IBA CODE")]
    pub iba_code: &'a str,

    #[serde(rename = "BCR CODE")]
    pub bcr_code: &'a str,

    #[serde(rename = "USFWS CODE")]
    pub usfws_code: &'a str,

    #[serde(rename = "ATLAS BLOCK")]
    pub atlas_block: &'a str,

    #[serde(rename = "LOCALITY")]
    pub locality: &'a str,

    #[serde(rename = "LOCALITY ID")]
    pub locality_id: &'a str,

    #[serde(rename = "LOCALITY TYPE")]
    pub locality_type: &'a str,

    #[serde(rename = "LATITUDE")]
    pub latitude: f64,

    #[serde(rename = "LONGITUDE")]
    pub longitude: f64,

    #[serde(rename = "OBSERVATION DATE")]
    pub observation_date: &'a str,

    #[serde(rename = "TIME OBSERVATIONS STARTED")]
    pub time_observations_started: &'a str,

    #[serde(rename = "OBSERVER ID")]
    pub observer_id: &'a str,

    #[serde(rename = "SAMPLING EVENT IDENTIFIER")]
    pub sampling_event_identifier: &'a str,

    #[serde(rename = "PROTOCOL TYPE")]
    pub protocol_type: &'a str,

    #[serde(rename = "PROTOCOL CODE")]
    pub protocol_code: &'a str,

    #[serde(rename = "PROJECT CODE")]
    pub project_code: &'a str,

    #[serde(rename = "DURATION MINUTES")]
    pub duration_minutes: &'a str,

    #[serde(rename = "EFFORT DISTANCE KM")]
    pub effort_distance_km: &'a str,

    #[serde(rename = "EFFORT AREA HA")]
    pub effort_area_ha: &'a str,

    #[serde(rename = "NUMBER OBSERVERS")]
    pub number_observers: &'a str,

    #[serde(
        rename = "ALL SPECIES REPORTED",
        deserialize_with = "deserialize_bool_from_u64"
    )]
    pub all_species_reported: bool,

    #[serde(rename = "GROUP IDENTIFIER")]
    pub group_identifier: &'a str,

    #[serde(rename = "HAS MEDIA", deserialize_with = "deserialize_bool_from_u64")]
    pub has_media: bool,

    #[serde(rename = "APPROVED", deserialize_with = "deserialize_bool_from_u64")]
    pub approved: bool,

    #[serde(rename = "REVIEWED", deserialize_with = "deserialize_bool_from_u64")]
    pub reviewed: bool,

    #[serde(rename = "REASON")]
    pub reason: &'a str,

    #[serde(rename = "TRIP COMMENTS")]
    pub trip_comments: &'a str,

    #[serde(rename = "SPECIES COMMENTS")]
    pub species_comments: &'a str,
}

pub fn deserialize_bool_from_u64<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct U64ToBoolVisitor;

    impl<'de> serde::de::Visitor<'de> for U64ToBoolVisitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer `0` or `1`")
        }

        fn visit_u64<E>(self, n: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match n {
                0 => Ok(false),
                1 => Ok(true),
                _ => panic!("TODO"),
            }
        }
    }

    deserializer.deserialize_any(U64ToBoolVisitor)
}

