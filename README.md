# Form API

Receives a captcha token from frontend, validate on Google Captcha endpoint and writes in the database.

## Quickstart

#### With Docker-compose

1. Make sure the `.env` variables are configured correctly, see [Environment variables](#environment-variables). The MYSQL_URL must match the `docker-compose.yml` database's environment variables

2. `make docker-build-rust` to build the project's image

3. `make docker-run`

#### Without Docker

1. Make sure you have Rust installed and a MySQL database running

2. Export the environment variables, see [Environment variables](#environment-variables)

3. Create the database and tables inside the `init.sql` file

3. cargo build

4. cargo run


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

1. basic

```bash
curl -d "cep=5354351&residentes=1&diagnostico=teste&recaptcha_response=03AGddagsdgja" -X POST http://localhost:8080/validate

```

2. With array parameter

```bash
curl -d "cep=22756501&logradouro=Rua+ABC&bairro=ttt&cidade=Rio+de+Janeiro&estado=RJ&residentes=7&sintomas%5B%5D=Febre&sintomas%5B%5D=Tosse&sintomas%5B%5D=NauseaEVomito&sintomas%5B%5D=PerdaDeOlfato&sintomas%5B%5D=PerdaDePaladar&diagnostico=teste&recaptcha_response=7f8sbewq" -X POST http://localhost:8080/validate
```

