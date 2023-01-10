use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
    result::Error,
    sql_query, PgConnection,
};

/// generate the test value of sturct
pub trait Test {
    fn test() -> Self
    where
        Self: Sized;
}

/// removes all tables
pub fn rollback_db(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Result<(), Error> {
    sql_query("DROP SCHEMA public CASCADE;").execute(conn)?;
    sql_query("CREATE SCHEMA public;").execute(conn)?;

    Ok(())
}
