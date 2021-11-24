#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use backend::*;
use backend::models::*;
use diesel::prelude::*;


#[post("/users/create/<name>")]
fn create_new_user(name: String) -> String {
    use schema::users;
    let conn = establish_connection();

    let new_user = NewUser { name: &name };

    new_user.insert_into(users::table)
        .get_result::<User>(&conn)
        .map(|x| format!("User: 'name: {}, id: {}' created.", x.name, x.id))
        .unwrap_or("Error saving new user.".to_string())

}

#[get("/users")]
fn get_users() -> String {
    let conn = establish_connection();

    use schema::users::dsl::*;
    let results = users.load::<User>(&conn)
        .expect("Error loading users.")
        .into_iter()
        .map(|x| format!("Name: {}, ID: {}", x.name, x.id))
        .collect::<Vec<_>>();

    results.join("\n")
}


fn main() {
    rocket::ignite()
        .mount("/", routes![get_users, create_new_user])
        .launch();
}
