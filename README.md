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
curl -d "cep=22753501&logradouro=Rua+Bacu&bairro=Abc&cidade=Rio+de+Janeiro&estado=RJ&trabSaude=y&idade=23&sexo=m&sintomas%5B%5D=DorDeCabeca&sintomas%5B%5D=PerdaDePaladar&sintomas%5B%5D=Diarreia&dataSintoma=2020-05-13&atendimentoMes=y&parenteConfirmado=n&casoSuspeito=nsi&casoConfirmado=y&recaptcha_response=aoidhbiuaf8392" -X POST http://localhost:8080/validate
```

