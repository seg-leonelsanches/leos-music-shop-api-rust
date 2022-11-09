use diesel::{prelude::*, result::Error};

use crate::models::microphone;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_all_microphones(
    conn: &mut SqliteConnection
) -> Result<Vec<microphone::Microphone>, DbError> {
    use crate::schema::microphones::dsl::*;

    let results = microphones
        .load::<microphone::Microphone>(conn)
        .expect("Error loading microphones");

    Ok(results)
}

/// Run query using Diesel to find microphone by id and return it.
pub fn find_microphone_by_id(
    conn: &mut SqliteConnection,
    microphone_id: i32,
) -> Result<Option<microphone::Microphone>, DbError> {
    use crate::schema::microphones::dsl::*;

    let microphone = microphones
        .filter(id.eq(microphone_id))
        .first::<microphone::Microphone>(conn)
        .optional()?;

    Ok(microphone)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_microphone(
    conn: &mut SqliteConnection,
    _model: &str, // prevent collision with `name` column imported inside the function
    _manufacturer: &str, 
    _description: &str
) -> Result<Option<i32>, Error> {
    // It is common when using Diesel with Actix Web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::microphones::dsl::*;

    /* let new_microphone = microphone::Microphone {
        id: Default::default(),
        model: _model.to_owned(),
        manufacturer: "".to_owned(),
        description: "".to_owned(),
        main_image: None
    }; */

    // diesel::insert_into(microphones).values(&new_microphone).execute(conn)?;
    diesel::insert_into(microphones)
        .values((model.eq(_model.to_owned()), manufacturer.eq(_manufacturer.to_owned()), description.eq(_description.to_owned())))
        .execute(conn)?;

    Ok(Some(1))
}