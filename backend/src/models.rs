use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub hashed_password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub hashed_password: &'a str,
}
