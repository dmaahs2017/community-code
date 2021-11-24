#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use backend::schema::posts::dsl::*;
use backend::*;
use backend::models::*;
use diesel::prelude::*;


#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}


fn main() {
    //rocket::ignite().mount("/hello", routes![hello]).launch();
    let connection = establish_connection();

    let post = create_post(&connection, "hello diesel", "This is my first diesel post");
    println!("\nSaved draft {} with id {}", post.title, post.id);

    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error handling posts");    

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}:{}", post.title, post.id);
        println!("{}", post.body);
        println!("==================================")
    }

}
