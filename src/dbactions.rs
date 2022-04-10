
use diesel::{expression_methods::*, QueryDsl};
use diesel::{PgConnection, RunQueryDsl};

use crate::founder::{NewFounder, Founder};
use crate::schema::founders::dsl;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn update_user(_conn: &PgConnection, new_founder: Founder) -> Result<Founder, DbError>
{
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    // use crate::schema::users::dsl::*;

    let _result = diesel::update(dsl::founders).set(&new_founder);
    Ok(new_founder)
}

pub fn insert_user(conn: &PgConnection, new_founder: NewFounder) -> Result<Founder, DbError>
{
    let founder = Founder::from(new_founder);

    diesel::insert_into(dsl::founders).values(&founder).execute(conn)?;
    Ok(founder)
}

pub fn delete_user(conn: &PgConnection, new_founder: Founder) -> Result<Founder, DbError>
{
    diesel::delete(dsl::founders.filter(dsl::uuid.eq(new_founder.uuid))).execute(conn)?;
    Ok(new_founder)
}

pub fn get_random_user(conn: &PgConnection, num_users: i64) -> Result<Vec<Founder>, DbError>
{
    no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

    // Usage, using the post schema from the getting started guide.
    let result = dsl::founders
        .order(RANDOM)
        .limit(num_users)
        .load(conn);
    Ok(result.unwrap())
}