use std::env;
use actix_web::{ post, App, HttpServer, middleware, client::Client, HttpResponse, web };
use serde_qs::Config;
use covid_survey::models::{ CaptchaResponse, Body, FormParameters, EnvData };
//use mysql::{ PooledConn };
//use covid_survey::database::{ get_db_connection };



#[post("/validate")]
async fn index(payload: String, data: web::Data<EnvData>) -> HttpResponse {

    let config = Config::new(10, false);
    let deserialized_params: Result<FormParameters, _> = config.deserialize_str(&payload);
    if let Ok(params) = deserialized_params {

        println!("\n\nparams: {:?}", params);
        let recaptcha_response: String = params.recaptcha_response.clone();

        let request_body = Body::new(data.captcha_secret.clone(), recaptcha_response);
        let client = Client::default();

        if let Ok(request_client) = client
            .post("https://www.google.com/recaptcha/api/siteverify")
            .content_type("application/x-www-form-urlencoded")
            .query(&request_body) {
                let response = request_client.send().await;
                if let Ok(mut data) = response {
                    if let Ok(parsed_data) = data.json::<CaptchaResponse>().await {
                        // validate
                        // check string and value constraints
                        println!("\nvalidated: {:?}", parsed_data);
                        return HttpResponse::Ok().json(parsed_data);
                    }
                }
            }
    }
    HttpResponse::InternalServerError().body("500 Internal error")
}



fn build_env_data() -> EnvData {
    let default_secret = String::from("myLittleSecret");
    let captcha_secret = env::var("CAPTCHA_SECRET").unwrap_or(default_secret);
    //let db_conn = get_db_connection().unwrap();
    EnvData {
        captcha_secret,
        //db_conn
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