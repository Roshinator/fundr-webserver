
pub mod founders
{
    use diesel::{expression_methods::*, QueryDsl};
    use diesel::{PgConnection, RunQueryDsl};

    use crate::founder::{NewFounder, Founder};

    pub fn update_user(conn: &PgConnection, new_founder: Founder) -> Result<Founder, diesel::result::Error>
    {
        use crate::schema::founders::dsl;
        // It is common when using Diesel with Actix Web to import schema-related
        // modules inside a function's scope (rather than the normal module's scope)
        // to prevent import collisions and namespace pollution.
        // use crate::schema::users::dsl::*;

        let result = diesel::update(dsl::founders.filter(dsl::id.eq(new_founder.id))).set(&new_founder).get_result(conn)?;
        Ok(result)
    }

    pub fn insert_user(conn: &PgConnection, new_founder: NewFounder) -> Result<Founder, diesel::result::Error>
    {
        use crate::schema::founders::dsl;

        let founder = Founder::from(new_founder);

        let _result = diesel::insert_into(dsl::founders).values(&founder).execute(conn)?;
        Ok(founder)
    }

    pub fn delete_user(conn: &PgConnection, id: uuid::Uuid) -> Result<uuid::Uuid, diesel::result::Error>
    {
        use crate::schema::founders::dsl;

        let _result = diesel::delete(dsl::founders.filter(dsl::id.eq(id))).execute(conn)?;
        Ok(id)
    }

    pub fn get_random_user(conn: &PgConnection, num_users: i64) -> Result<Vec<Founder>, diesel::result::Error>
    {
        use crate::schema::founders::dsl;

        no_arg_sql_function!(RANDOM, (), "Represents the sql RANDOM() function");

        // Usage, using the post schema from the getting started guide.
        let result = dsl::founders
            .order(RANDOM)
            .limit(num_users)
            .load(conn)?;
        Ok(result)
    }
}