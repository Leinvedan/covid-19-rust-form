use std::env;
use mysql::*;



pub fn get_db_connection() -> Result<PooledConn> {
  let url = env::var("MYSQL_URL").unwrap();
  let pool = Pool::new(url).unwrap();
  pool.get_conn()
}

