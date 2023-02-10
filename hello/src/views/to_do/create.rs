use crate::diesel;
use diesel::prelude::*;

// use serde_json::value::Value;
// use serde_json::Map;
use actix_web::{HttpRequest, HttpResponse};

// use crate::to_do::{to_do_factory, enums::TaskStatus};
use crate::json_serialization::to_do_items::ToDoItems;
// use crate::state::read_file;
// use crate::processes::process_input;

// use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;
use crate::schema::to_do;
use crate::database::DB;
use crate::jwt::JwToken;


pub async fn create(req: HttpRequest, token: JwToken, db: DB) -> HttpResponse {

    // use JSON file
    // let state: Map<String, Value> = read_file("./state.json");  // step 1
    // let title: String = req.match_info().get("title").unwrap().to_string(); // step 2
    // println!("{}", title);
    // let item = to_do_factory(&title.as_str(), TaskStatus::PENDING); // step 3
    
    // // let item_done = to_do_factory(&title.as_str(), TaskStatus::DONE);
    // process_input(item, "create".to_string(), &state); // step 4
    // // process_input(item_done, "create".to_string(), &state);
    // return HttpResponse::Ok().json(ToDoItems::get_state())

    // use Database
    let title: String = req.match_info().get("title").unwrap().to_string(); // step 2
    println!("{}", title);
    
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&db.connection)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id);
        let _ = diesel::insert_into(to_do::table).values(&new_post)
            .execute(&db.connection);
    }
    return HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}