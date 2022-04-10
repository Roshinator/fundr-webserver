
use uuid::Uuid;
use serde::{Serialize, Deserialize, de::Visitor};
use crate::schema::founders;
use diesel::{*};

// #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
// #[derive(AsExpression, FromSqlRow)]
// #[sql_type = "diesel::sql_types::Uuid"]
// pub struct FounderUuid(Uuid);
// impl FounderUuid
// {
//     pub fn new() -> FounderUuid
//     {
//         FounderUuid(Uuid::new_v4())
//     }
// }

#[derive(Serialize, Deserialize, Clone, Queryable, Insertable, Debug, AsChangeset, Identifiable)]
#[primary_key(id)]
pub struct Founder
{
    pub id: Uuid,
    pub name: String,
    pub company_name: String,
    pub bio: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewFounder
{
    pub name: String,
    pub company_name: String,
    pub bio: String,
    pub image: String,
}

impl From<NewFounder> for Founder
{
    fn from(nf: NewFounder) -> Self
    {
        Founder
        {
            id: Uuid::new_v4(),
            name: nf.name,
            company_name: nf.company_name,
            bio: nf.bio,
            image: nf.image
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