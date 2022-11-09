use actix_web::{get, post, web, Error, HttpResponse, Responder, Result};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/users/{user_id}/{friend}")] // <- define path parameters
async fn index(path: web::Path<(u32, String)>) -> Result<String> {
    let (user_id, friend) = path.into_inner();
    Ok(format!("Welcome {}, user_id {}!", friend, user_id))
}

#[get("/microphones")]
async fn get_all_microphones(pool: web::Data<super::db::DbPool>) -> Result<HttpResponse, Error> {
    let microphones = web::block(move || {
        let mut conn = pool.get()?;
        crate::actions::find_all_microphones(&mut conn)
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
        crate::actions::find_microphone_by_id(&mut conn, microphone_id)
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
    pool: web::Data<super::db::DbPool>,
    form: web::Json<crate::models::microphone::NewMicrophone>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let microphone = web::block(move || {
        let mut conn = pool.get()?;
        crate::actions::insert_new_microphone(&mut conn, &form.model)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(microphone))
}