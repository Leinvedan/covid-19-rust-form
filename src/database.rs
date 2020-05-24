use std::env;
use crate::models::{ FormParameters, NewFormData };

use diesel::prelude::*;
use diesel::mysql::MysqlConnection;


pub fn get_db_connection() -> MysqlConnection {
  let url = env::var("MYSQL_URL")
    .expect("MYSQL_URL must be set");
  MysqlConnection::establish(&url)
    .expect("Error connecting to MYSQL_URL")
}


pub fn insert_form_data(conn: &MysqlConnection, data: FormParameters) -> bool {
  use crate::schema::pessoa::dsl::*;

  let new_form_data: NewFormData = NewFormData::from_form(data);

  diesel::insert_into(pessoa)
        .values(&new_form_data)
        .execute(conn)
        .is_ok()
}