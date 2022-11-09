use actix_web::{get, post, web, Error, HttpResponse, Result};

#[get("/microphones")]
async fn get_all_microphones(pool: web::Data<crate::db::DbPool>) -> Result<HttpResponse, Error> {
    let microphones = web::block(move || {
        let mut conn = pool.get()?;
        crate::actions::microphones::find_all_microphones(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(microphones))
}

/// Finds microphone by ID.
#[get("/microphones/{microphone_id}")]
async fn get_microphone(
    pool: web::Data<crate::db::DbPool>,
    microphone_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let microphone_id = microphone_id.into_inner();

    // use web::block to offload blocking Diesel code without blocking server thread
    let microphone = web::block(move || {
        let mut conn = pool.get()?;
        crate::actions::microphones::find_microphone_by_id(&mut conn, microphone_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(microphone) = microphone {
        Ok(HttpResponse::Ok().json(microphone))
    } else {
        let res = HttpResponse::NotFound().body(format!("No microphone found with id: {microphone_id}"));
        Ok(res)
    }
}

#[post("/microphones")]
async fn add_microphone(
    pool: web::Data<crate::db::DbPool>,
    form: web::Json<crate::models::microphone::NewMicrophone>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || {
        let mut conn = pool.get().unwrap();
        crate::actions::microphones::insert_new_microphone(&mut conn, &form.model, &form.manufacturer, &form.description)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().into())
}