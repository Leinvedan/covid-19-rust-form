use std::env;
use actix_web::{ post, App, HttpServer, middleware, client::Client, HttpResponse, web, Error };
use serde_qs::Config;
use diesel::mysql::MysqlConnection;
use covid_survey::models::{ CaptchaResponse, Body, FormParameters, EnvData };
use covid_survey::database::{ get_db_connection, insert_form_data };
use log::{ info, error };
use env_logger::Env;


fn log_and_return_http<T>(_: T, log: &str) -> HttpResponse {
    error!("{}", log);
    HttpResponse::InternalServerError().body("500 Internal error")
}


#[post("/validate")]
async fn index(payload: String, env_data: web::Data<EnvData>) -> Result<HttpResponse, Error> {

    let config = Config::new(10, false);
    let deserialized_params: Result<FormParameters, _> = config.deserialize_str(&payload);
    let deserialized_form = deserialized_params
        .map_err(|err| log_and_return_http(err, "Unable to desserialize"))?;

    let recaptcha_response: String = deserialized_form.recaptcha_response.clone();

    let request_body = Body::new(env_data.captcha_secret.clone(), recaptcha_response);
    let client = Client::default();

    let request_client = client
      .post("https://www.google.com/recaptcha/api/siteverify")
      .content_type("application/x-www-form-urlencoded")
      .query(&request_body)
      .map_err(|err| log_and_return_http(err, "Unable to parse Body into query"))?;
    let mut response = request_client
        .send()
        .await
        .map_err(|err| log_and_return_http(err, "Unable connect to Google API"))?;
    let parsed_data = response.json::<CaptchaResponse>()
        .await
        .map_err(|err| log_and_return_http(err, "Unable to parse Google API Response"))?;
    info!("SCORE={}", parsed_data.score.unwrap_or(0.0));
    if parsed_data.score.unwrap_or(0.0) >= 0.5 {
        let inserted: bool = insert_form_data(&env_data.db_conn, deserialized_form);
        info!("INSERTED={}", inserted);
    }

    Ok(HttpResponse::Ok().body(
        "<h1>Obrigado por participar!</br>Sua ajuda foi fundamental no combate ao COVID-19!</h1>"))
}



fn build_env_data() -> EnvData {
    let default_secret = String::from("myLittleSecret");
    let captcha_secret = env::var("CAPTCHA_SECRET").unwrap_or(default_secret);
    let db_conn: MysqlConnection = get_db_connection();
    info!("Connected to database");
    EnvData {
        captcha_secret,
        db_conn
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let log_env = Env::default()
        .filter_or("MY_LOG_LEVEL", "info")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(log_env);

    let default_port = String::from("8080");
    let app_port = env::var("PORT").unwrap_or(default_port);
    let addr = format!("0.0.0.0:{}", app_port);

    let server = HttpServer::new(|| { App::new()
            .data(build_env_data())
            .wrap(middleware::Logger::default())
            .service(index)
        })
        .bind(&addr)?;
    info!("Starting http server: {}", &addr);
    server.run().await
}