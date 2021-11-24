#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use backend::*;
use backend::models::*;
use diesel::prelude::*;
use argon2::Argon2;
use argon2::password_hash::{
    rand_core::OsRng,
    PasswordHash, PasswordHasher, PasswordVerifier, SaltString
};


#[post("/users/create/<name>/<password>")]
fn create_new_user(name: String, password: String) -> String {
    use schema::users;
    let conn = establish_connection();

    //TODO: Perhaps this should be hashed client side,
    // otherwise its vulnerable to sniffing upon user creation.
    // For now I will hash server side because there is no front end yet.
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();
    // Hash password to PHC string ($argon2id$v=19$...)
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
    // Verify password against PHC string
    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
    assert!(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok());
    
    let new_user = NewUser { name: &name, hashed_password: &hashed_password };

    new_user.insert_into(users::table)
        .get_result::<User>(&conn)
        .map(|x| format!("User: 'name: {}, id: {}' created.", x.name, x.id))
        .unwrap_or("Error saving new user.".to_string())

}

#[get("/users")]
fn get_users() -> String {
    let conn = establish_connection();
    use schema::users::dsl::*;
    users.load::<User>(&conn)
        .map(|result| {
            result.into_iter()
            .map(|x| format!("Name: {}, ID: {}, Hashword: {}", x.name, x.id, x.hashed_password))
            .collect()
        })
        .unwrap_or("Error loading users.".to_string())
}


fn main() {
    rocket::ignite()
        .mount("/", routes![get_users, create_new_user])
        .launch();
}
