# Table of contents

1. [Running the project](#running-the-project)

2. [Client](#client)

3. [API](#API)

4. [Adding new Form fields](#adding-new-form-fields)

# Running the project

## **BEFORE RUNNING THE PROJECT**
This project uses Google reCaptcha v3. Before running the project, you need to get your domain keys [HERE](http://www.google.com/recaptcha/admin) and set them:

 1. `.env` file -> Secret Key

 2. `CAPTCHA_KEY` field inside `client/src/js/captchaLogic.js` -> Website Key

        Be careful not to commit these keys

## *Quickstart*

1. make sure docker and docker-compose are installed.

2. run `make docker-build-all` and grab a cup of coffe, this will take some time...

3. run `make run` and you're done!

4. The Form client is working on `http://127.0.0.1:5500/` and your API can receive POST request on `http://127.0.0.1:8080/validate`

# Client

The client includes a HTML Form with Google reCaptcha v3. The ReCaptcha client token is in the `dist index.js file`.
To update the token: add your token to `CAPTCHA_KEY` inside the `src/js/captchaLogic.js` and build the static files using gulp or run `make docker-build-client` if using Docker.

### Running locally (run the commands inside the client directory)

1. install npm

2. run `npm install`

3. run `npm run build` to build your static files into `/dist` (after that, you can use `npm run watch` to automatically build your files as you change them)

4. run `npm run start` to lauch the server at `http://127.0.0.1:5500/`

# API

Receives a captcha token from the frontend, validate on Google reCaptcha endpoint and writes in the database.

## API Quickstart

#### With Docker-compose

1. Make sure the `.env` variables are configured correctly, see [Environment variables](#environment-variables). The MYSQL_URL must match the `docker-compose.yml` database's environment variables

2. `make docker-build-api` to build the project's image

3. `make docker-run`

#### Without Docker

1. Make sure Rust is installed and the MySQL database is running

2. Export the environment variables, see [Environment variables](#environment-variables)

3. Create the database and tables (sames as the ones inside the `init.sql` file)

3. `cargo build` or cargo `build --release` for production

4. `cargo run`


## Environment variables

| Name           | Description                 |
|----------------|-----------------------------|
| PORT           | POST API port               |
| CAPTCHA_SECRET | Google ReCaptcha secret key |
| MYSQL_URL      | MYSQL Database Url          |

Examples:

```bash
export PORT=8080
export CAPTCHA_SECRET=mySecret
export MYSQL_URL="mysql://root:password@127.0.0.1:3307/covidForm"
```

## Example POST request

```bash
curl -d "cep=22753501&logradouro=Rua+Bacu&bairro=Abc&cidade=Rio+de+Janeiro&estado=RJ&trabSaude=y&idade=23&sexo=m&sintomas%5B%5D=DorDeCabeca&sintomas%5B%5D=PerdaDePaladar&sintomas%5B%5D=Diarreia&dataSintoma=2020-05-13&atendimentoMes=y&parenteConfirmado=n&casoSuspeito=nsi&casoConfirmado=y&recaptcha_response=aoidhbiuaf8392" -X POST http://localhost:8080/validate
```

# Adding new Form fields

This can be a little tricky:

1. add the new field to the HTML file. Example:

```HTML
<label for="cidade" class="question" >Cidade</label><p class="red-icon">*</p>
<input id="cidade" type="text" name="cidade" readonly="readonly" maxlength="250" required/>
```

2. Go to `api/src/models.rs` add your new field name to `FormParameters` and `NewFormData`. **The name must be the same of the HTML tag**. Example:

```Rust
pub struct FormParameters {
    pub cep: String,
    pub logradouro: String,
    pub bairro: String,
    pub cidade: String <--- HERE
    // if the field is optional, use this instead
    pub cidade: Option<String> 
}

pub struct NewFormData {
    cep: String,
    logradouro: String,
    bairro: String,
    cidade: String <--- HERE
    // if the field is optional, use this instead
    pub cidade: Option<String>
}
```

3. Update the schema inside `api/src/schema.rs` with your new field, the types used inside these files are the same used in the MySQL database, for more see [Diesel Docs](http://diesel.rs/). Example:

```Rust
table! {
  pessoa (id) {
      id -> Unsigned<Integer>,
      cep -> Text,
      logradouro -> Text,
      bairro -> Text,
      cidade -> Text, <--- HERE
      // if the field is optional, use this instead
      cidade -> Nullable<Text>
  }
}
```

4. Update you database table `pessoa`. Example updating the `init.sql` which is used with docker-compose:

```SQL
CREATE TABLE IF NOT EXISTS pessoa (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    cep TEXT,
    logradouro TEXT NOT NULL,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL --HERE
    PRIMARY KEY (id)
);
```


