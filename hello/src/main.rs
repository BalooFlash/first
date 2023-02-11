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
mod counter;


// #[actix_web::main]
// async fn main() -> std::io::Result <()> {
//     const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    
//     let mut site_counter = counter::Counter{count: 0};
//     site_counter.save();
//     println!("first");
//     HttpServer::new( || {
//         let cors = Cors::default().allow_any_origin()
//                                 .allow_any_method()
//                                 .allow_any_header();
//     println!("second");

//         let app = App::new()
//             .wrap_fn(|req, srv| {
//                 let passed: bool;
//     println!("third");
                
//                 let mut site_counter = counter::Counter::load().unwrap();
//                 // site_counter += 1;
//                 println!("site_counter: {:?}", &site_counter);
//                 site_counter.save();

//                 if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
//                     passed = true;
//                 } else {
//                     passed = false;
//                 }
//                 println!("{}", passed);
//                 let end_result;
//                 if passed == true {
//                     end_result = Either::Left(srv.call(req));
//                 }else {
//                     let resp = HttpResponse::NotImplemented().body(format!("only {} API is no longer supported", ALLOWED_VERSION));
//                     end_result = Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
//                 }
//                 async move {
//                     let result = end_result.await?;
//                     Ok(result)
//                 }
//             }).configure(views::views_factory).wrap(cors);
//             return app
//     })
//     .bind("127.0.0.1:8000")?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    let site_counter = counter::Counter{count: 0};
    let _ = site_counter.save();

    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv|{

                let passed: bool;
                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                println!("{:?}", &site_counter);
                let _ = site_counter.save();

                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                }

                println!("{:?}", req);
                let end_result = match passed {
                    true => {
                        Either::Left(srv.call(req))
                    },
                    false => {
                        let resp = HttpResponse::NotImplemented().body(format!(
                            "only {} API is supported", ALLOWED_VERSION
                        ));
                        Either::Right(
                            ok(req.into_response(resp).map_into_boxed_body())
                        )
                    }
                };
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
