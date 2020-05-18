use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct CaptchaResponse {
    pub success: Option<bool>,
    pub score: Option<f32>,
    pub action: Option<String>,
    pub challenge_ts: Option<String>,
    pub hostname: Option<String>,
    pub error_codes: Option<Vec<i32>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    pub secret: String,
    pub response: String,
}

impl Body {
    pub fn new(secret: String, response: String) -> Self {
        Body {
            secret,
            response
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormParameters {
    pub cep: String,
    pub logradouro: Option<String>,
    pub bairro: Option<String>,
    pub cidade: Option<String>,
    pub estado: Option<String>,
    pub residentes: Option<i32>,
    pub sintomas: Option<Vec<String>>,
    pub diagnostico: Option<String>,
    pub recaptcha_response: String
}

pub struct EnvData {
  pub captcha_secret: String,
  //db_conn: PooledConn
}