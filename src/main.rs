use std::env;
use actix_web::{ post, App, HttpServer, middleware, client::Client, HttpResponse, web };
use serde_qs::Config;
use diesel::mysql::MysqlConnection;
use covid_survey::models::{ CaptchaResponse, Body, FormParameters, EnvData };
use covid_survey::database::{ get_db_connection, insert_form_data };



#[post("/validate")]
async fn index(payload: String, env_data: web::Data<EnvData>) -> HttpResponse {

    let config = Config::new(10, false);
    let deserialized_params: Result<FormParameters, _> = config.deserialize_str(&payload);
    if let Ok(deserialized_form) = deserialized_params {

        println!("\n\nparams: {:?}", deserialized_form);
        //let recaptcha_response: String = deserialized_form.recaptcha_response.clone();

        // TEMP!!!
        let inserted: bool = insert_form_data(&env_data.db_conn, deserialized_form);
        println!("INSETED? = {}", inserted);
        // TEMP END!!!!

        //let request_body = Body::new(env_data.captcha_secret.clone(), recaptcha_response);
        //let client = Client::default();

        // if let Ok(request_client) = client
        //     .post("https://www.google.com/recaptcha/api/siteverify")
        //     .content_type("application/x-www-form-urlencoded")
        //     .query(&request_body) {
        //         let response = request_client.send().await;
        //         if let Ok(mut data) = response {
        //             if let Ok(parsed_data) = data.json::<CaptchaResponse>().await {
                        
        //                 if parsed_data.score.unwrap_or(0.0) >= 0.5 {
        //                     let inserted: bool = insert_form_data(&env_data.db_conn, deserialized_form);
        //                     println!("INSETED? = {}", inserted);
        //                 }

        //                 return HttpResponse::Ok().body(
        //                     "<h1>Obrigado por participar! Você foi fundamental no combate ao COVID-19!</h1>"
        //                 );
        //             }
        //         }
        //     }
    }
    HttpResponse::InternalServerError().body("500 Internal error")
}



fn build_env_data() -> EnvData {
    let default_secret = String::from("myLittleSecret");
    let captcha_secret = env::var("CAPTCHA_SECRET").unwrap_or(default_secret);
    let db_conn: MysqlConnection = get_db_connection();
    println!("Connected to database");
    EnvData {
        captcha_secret,
        db_conn
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let default_port = String::from("8080");
    let app_port = env::var("PORT").unwrap_or(default_port);
    let addr = format!("0.0.0.0:{}", app_port);

    std::env::set_var("RUST_LOG", "actix_web=info");
    let server = HttpServer::new(|| { App::new()
            .data(build_env_data())
            .wrap(middleware::Logger::default())
            .service(index)
        })
        .bind(&addr)?;
    println!("Starting http server: {}", &addr);
    server.run().await
}