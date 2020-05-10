use std::env;
use actix_web::{get, web, App, HttpServer, Responder, middleware, client::Client};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaResponse {
    success: Option<bool>,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    error_codes: Option<Vec<i32>> 
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    client_id: String,
    client_secret: String,
    code: String,
    accept: String,
}


impl Body {
    fn new(client_id: String, client_secret: String, code: String, accept: String) -> Self {
        Body {
            client_id,
            client_secret,
            code,
            accept,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TestResponse {
    json: Body
}

// https://www.google.com/recaptcha/api/siteverify

#[get("/validate/{token}")]
async fn index(captcha_token: web::Path<String>) -> impl Responder {
    let session_code: String = "session_code".to_string();
    //let app_secret = env::var("SECRET").unwrap();
    let app_secret: String = "asdasdwosamsoafma4324das2234".to_string();

    let json_body = Body::new(captcha_token.to_string(), app_secret, session_code, String::from("json"));
    let client = Client::default();

    let res = client.post("https://postman-echo.com/post")
        .send_json(&json_body)
        .await;
    match res {
        Ok(mut data) => {
            let parsed = data.json::<TestResponse>().await;
            match parsed {
                Ok(parsed_data) => format!("{:?}", parsed_data),
                Err(_) => "ERROR PARSING DATA!!!!".to_string()
            }
        }
        Err(_) => "ERROR SENDING REQUEST".to_string()
    }
}
    


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(|| {App::new()
        .wrap(middleware::Logger::default())
        .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}