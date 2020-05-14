# Form API

Receives a captcha token from frontend, validate on Google Captcha endpoint and returns response.

### Example POST request

```bash
curl -d "cep=5354351&residentes=1&diagnostico=teste&recaptcha_response=03AGddagsdgja" -X POST http://localhost:8080/validate

```

With array parameter

```bash
curl -d "cep=22756501&logradouro=Rua+ABC&bairro=ttt&cidade=Rio+de+Janeiro&estado=RJ&residentes=7&sintomas%5B%5D=Febre&sintomas%5B%5D=Tosse&sintomas%5B%5D=NauseaEVomito&sintomas%5B%5D=PerdaDeOlfato&sintomas%5B%5D=PerdaDePaladar&diagnostico=teste&recaptcha_response=7f8sbewq" -X POST http://localhost:8080/validate
```

