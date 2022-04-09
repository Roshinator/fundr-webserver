use std::path::PathBuf;
use uuid::Uuid;
use serde::{Serialize, Deserialize, de::Visitor};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct FounderUuid(Uuid);
impl FounderUuid
{
    pub fn new() -> FounderUuid
    {
        FounderUuid(Uuid::new_v4())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Founder
{
    pub uuid: FounderUuid,
    pub name: String,
    pub company_name: String,
    pub bio: String,
    pub image: PathBuf,
}

impl Serialize for FounderUuid
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_u128(self.0.as_u128())
    }
}

impl<'de> Deserialize<'de> for FounderUuid
{
    fn deserialize<D>(deserializer: D) -> Result<FounderUuid, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        match deserializer.deserialize_u128(U128Visior)
        {
            Ok(val) => Ok(FounderUuid(Uuid::from_u128(val))),
            Err(x) => Err(x)
        }
    }
}

struct U128Visior;
impl<'de> Visitor<'de> for U128Visior
{
    type Value = u128;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        formatter.write_str("an integet between 0 and 2^128")
    }

    fn visit_u128<E>(self, value: u128) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(value)
    }
}