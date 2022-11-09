use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;