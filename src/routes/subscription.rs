use actix_web::dev::Server;
use actix_web::{web, App, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
