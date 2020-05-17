use std::env;
use actix_web::{post, App, HttpServer, middleware, client::Client, HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_qs::Config;



#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaResponse {
    success: Option<bool>,
    score: Option<f32>,
    action: Option<String>,
    challenge_ts: Option<String>,
    hostname: Option<String>,
    error_codes: Option<Vec<i32>>
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    secret: String,
    response: String,
}


impl Body {
    fn new(secret: String, response: String) -> Self {
        Body {
            secret,
            response
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormParameters {
    cep: String,
    logradouro: Option<String>,
    bairro: Option<String>,
    cidade: Option<String>,
    estado: Option<String>,
    residentes: Option<i32>,
    sintomas: Option<Vec<String>>,
    diagnostico: Option<String>,
    recaptcha_response: String
}


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
                        println!("\nvalidated: {:?}", parsed_data);
                        return HttpResponse::Ok().json(parsed_data);
                    }
                }
            }
    }
    HttpResponse::InternalServerError().body("500 Internal error")
}

struct EnvData {
    captcha_secret: String,
}

fn build_env_data() -> EnvData {
    let captcha_secret = match env::var("CAPTCHA_SECRET") {
        Ok(secret) => secret,
        Err(_e) => "myLittleSecret".to_string(),
    };
    EnvData{
        captcha_secret
    }
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let app_port = match env::var("PORT") {
        Ok(port) => port,
        Err(_e) => "8080".to_string(),
    };
    let addr = format!("0.0.0.0:{}", app_port);

    
    println!("Starting http server: {}", &addr);

    std::env::set_var("RUST_LOG", "actix_web=info");
    HttpServer::new(|| { App::new()
        .data(build_env_data())
        .wrap(middleware::Logger::default())
        .service(index)
    })
    .bind(&addr)?
    .run()
    .await
}