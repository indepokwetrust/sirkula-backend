use core::fmt;

use chrono::NaiveDateTime;

pub struct Entity {
    pub display_name: Varchar<100>,
    pub password_hash: Varchar<100>,
    pub entity_type: EntityType,
    pub contact_number: Varchar<40>,
    pub email: Varchar<40>,
    pub website_url: Option<Varchar<400>>,
    pub created_on: NaiveDateTime,
    pub deleted_on: Option<NaiveDateTime>,
}

pub struct DonationPost {
    pub donator_entity_id: U63,
    pub item_name: Varchar<40>,
    pub category: Varchar<40>,
    pub pickup_location_latitude: i64,
    pub pickup_location_longitude: i64,
    pub pickup_address: Varchar<100>,
    pub pickup_city: Varchar<100>,
    pub pickup_location_note: Option<Varchar<100>>,
    pub amount: U63,
    pub measurement_unit: MeasurementUnit,
    pub description: Option<Varchar<400>>,
    pub created_on: NaiveDateTime,
    pub deleted_on: Option<NaiveDateTime>,
    pub completed_on: Option<NaiveDateTime>,
}

pub struct Request {
    pub requester_entity_id: U63,
    pub donation_post_id: U63,
    pub intent: Option<Varchar<400>>,
    pub created_on: NaiveDateTime,
    pub deleted_on: Option<NaiveDateTime>,
    pub accepted_on: Option<NaiveDateTime>,
    pub completed_on: Option<NaiveDateTime>,
}

pub struct DonationPostImage {
    pub donation_post_id: U63,
    pub image_url: Varchar<100>,
}

pub struct ProfileImage {
    pub entity_id: U63,
    pub image_url: Varchar<100>,
}

pub enum EntityType {
    UMKM,
    Komunitas,
    Perusahaan,
}
impl EntityType {
    pub fn to_string(&self) -> String {
        match self {
            Self::UMKM => "UMKM",
            Self::Komunitas => "Komunitas",
            Self::Perusahaan => "Perusahaan",
        }
        .to_string()
    }
}

pub enum MeasurementUnit {
    GRAM,
    PCS,
}
impl MeasurementUnit {
    pub fn to_string(&self) -> String {
        match self {
            Self::GRAM => "gram",
            Self::PCS => "pcs",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct CastingError;
impl std::error::Error for CastingError {}
impl fmt::Display for CastingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to cast safely")
    }
}

pub struct Varchar<const N: usize>(String);
impl<const N: usize> TryFrom<String> for Varchar<N> {
    type Error = CastingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= N {
            Ok(Varchar::<N>(value))
        } else {
            Err(CastingError)
        }
    }
}

impl<const N: usize> From<Varchar<N>> for String {
    fn from(value: Varchar<N>) -> Self {
        value.0
    }
}

pub struct U63(i64);
impl TryFrom<i64> for U63 {
    type Error = CastingError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(U63(value))
        } else {
            Err(CastingError)
        }
    }
}
impl TryFrom<u64> for U63 {
    type Error = CastingError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value <= 0x7fffffffffffffff {
            Ok(U63(value.try_into().unwrap()))
        } else {
            Err(CastingError)
        }
    }
}

impl From<U63> for i64 {
    fn from(value: U63) -> Self {
        value.0
    }
}
impl From<U63> for u64 {
    fn from(value: U63) -> Self {
        value.0.try_into().unwrap()
    }
}
