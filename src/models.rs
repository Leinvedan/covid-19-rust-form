use serde::{Deserialize, Serialize};
use crate::schema::pessoa;
use diesel::mysql::MysqlConnection;


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


pub struct EnvData {
    pub captcha_secret: String,
    pub db_conn: MysqlConnection
}


#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FormParameters {
    pub cep: String,
    pub logradouro: String,
    pub bairro: String,
    pub cidade: String,
    pub estado: String,
    pub trabSaude: Option<String>,
    pub idade: u32,
    pub sexo: Option<String>,
    pub sintomas: Option<Vec<String>>,
    pub dataSintoma: Option<String>,
    pub atendimentoMes: String,
    pub parenteConfirmado: String,
    pub casoSuspeito: String,
    pub casoConfirmado: String,
    pub recaptcha_response: String
}

impl NewFormData {
    fn parse_symptoms_to_u32(symptoms: Vec<String>) -> u32 {
        symptoms
          .iter()
          .map(
            |symp| match symp.as_str() {
              // total: 12 | map symptoms to an unsiged int
              "Febre" => 0x001, //0000 0000 0001
              "Tosse" => 0x002, //0000 0000 0010
              "Espirro" => 0x004,
              "Coriza" => 0x008,
              "DorDeGarganta" => 0x010,
              "DorDeCabeca" => 0x020,
              "Enjoo" => 0x040,
              "DificuldadeDeRespirar" => 0x080,
              "PerdaDePaladar" => 0x100,
              "PerdaDeOlfato" => 0x200,
              "Diarreia" => 0x400,
              "DoresNocorpo" => 0x800,
              _ => 0
            }
          )
          .fold(0, |acc, next| acc & next)
    }

    pub fn from_form(form_param: FormParameters) -> NewFormData {
        let empty_symp = vec![String::from("")];
        let symp: Vec<String> = form_param.sintomas.unwrap_or(empty_symp);
        let parsed_symp: u32 = NewFormData::parse_symptoms_to_u32(symp);
        NewFormData {
            cep: form_param.cep,
            logradouro: form_param.logradouro,
            bairro: form_param.bairro,
            cidade: form_param.cidade,
            estado: form_param.estado,
            trabSaude: form_param.trabSaude,
            idade: form_param.idade,
            sexo: form_param.sexo,
            sintomas: parsed_symp,
            dataSintoma: form_param.dataSintoma,
            atendimentoMes: form_param.atendimentoMes,
            parenteConfirmado: form_param.parenteConfirmado,
            casoSuspeito: form_param.casoSuspeito,
            casoConfirmado: form_param.casoConfirmado,
            recaptcha_response: form_param.recaptcha_response
        }
    }
}


#[allow(non_snake_case)]
#[derive(Insertable)]
#[table_name="pessoa"]
pub struct NewFormData {
    cep: String,
    logradouro: String,
    bairro: String,
    cidade: String,
    estado: String,
    trabSaude: Option<String>,
    idade: u32,
    sexo: Option<String>,
    sintomas: u32,
    dataSintoma: Option<String>,
    atendimentoMes: String,
    parenteConfirmado: String,
    casoSuspeito: String,
    casoConfirmado: String,
    recaptcha_response: String
}



