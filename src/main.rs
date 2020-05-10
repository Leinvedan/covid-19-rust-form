// use std::env;
use actix_web::{post, web, App, HttpServer, Responder, middleware, client::Client, HttpResponse};
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

#[derive(Serialize, Deserialize, Debug)]
pub struct PostParameters {
    captcha_token: String,
}

// https://www.google.com/recaptcha/api/siteverify

#[post("/validate")]
async fn index(params: web::Json<PostParameters>) -> impl Responder {
    println!("received: {:?}", params);
    let captcha_token: String = params.captcha_token.clone();
    let session_code: String = "session_code".to_string();
    //let app_secret = env::var("SECRET").unwrap();
    let app_secret: String = "asdasdwosamsoafma4324das2234".to_string();

    let json_body = Body::new(captcha_token, app_secret, session_code, String::from("json"));
    let client = Client::default();

    let response = client.post("https://postman-echo.com/post")
        .send_json(&json_body)
        .await;
    if let Ok(mut data) = response {
        if let Ok(parsed_data) = data.json::<TestResponse>().await {
            // validate
            return HttpResponse::Ok().json(parsed_data);
        }
    }
    HttpResponse::InternalServerError().body("500 Internal error")
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(|| { App::new()
        .wrap(middleware::Logger::default())
        .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}