use actix_web::{web, middleware, Scope, dev::HttpServiceFactory};
mod controllers;

fn users() -> impl HttpServiceFactory {
    web::scope("/users").service(controllers::users::index)
}

pub fn route() -> impl HttpServiceFactory {
    web::scope("/api/v1")
        .wrap(middleware::NormalizePath)
        .route("", web::get().to(controllers::index))
        .service(users())
}
