#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, HttpResponse};
use actix_service::Service;
use futures::future::{ok, Either};
use actix_cors::Cors;

mod schema;
mod database;
mod views;
mod to_do;
// mod state;  
// mod processes;
mod json_serialization;
mod jwt;
mod models;
mod config;


#[actix_web::main]
async fn main() -> std::io::Result <()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");

    HttpServer::new( || {
        let cors = Cors::default().allow_any_origin()
                                .allow_any_method()
                                .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("{}", OUTCOME);
                let passed: bool;

                if *&req.pat().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = false;
                } else {
                    passed = true;
                }

                let end_result;
                if passed == true {
                    end_result = Either::Left(srv.call(req));
                }else {
                    let resp = HttpResponse::NotImplemented().body("only {} API is no longer supported", ALLOWED_VERSION);
                    end_result = Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                }
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            }).configure(views::views_factory).wrap(cors);
            return app
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}