use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use futures::{StreamExt, TryStreamExt};
// use std::fs;
// use async_std::prelude::*;
// use sanitize_filename;

mod database;

#[get("/drops")]
async fn index() -> impl Responder {
    database::create_database(); //in-case database not created.
    HttpResponse::Ok().body("Drop#index")
}



#[post("/drops")]
async fn create(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    database::create_database(); //in-case database not created.

    let mut video_data = Vec::new();

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            video_data.extend_from_slice(&data);
        }
    }

    // Pass the video data to the add_video function
    let hash = database::add_video(&video_data)?;

    Ok(HttpResponse::Ok().body(format!("Drop#new {}", hash)))
}

#[get("/drops/{id}")]
async fn show(id: web::Path<String>) -> impl Responder {
    database::create_database(); //in-case database not created.

    let video_data = match database::get_video(id.to_string()){
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };



    HttpResponse::Ok().header("Content-Type", "video/mp4").body(video_data)
}

#[put("/drops/{id}")]
async fn update(id: web::Path<String>) -> impl Responder {
    database::create_database(); //in-case database not created.
    HttpResponse::Ok().body(format!("Drop#update {}", id))
}

#[delete("/drops/{id}")]
async fn destroy(id: web::Path<String>) -> impl Responder {
    database::create_database(); //in-case database not created.
    HttpResponse::Ok().body(format!("Drop#delete {}", id))
}
