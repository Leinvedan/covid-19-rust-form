# Form API

Receives a captcha token from frontend, validate on Google Captcha endpoint and returns response.

### Example POST request

```bash
curl -d "cep=5354351&residentes=1&diagnostico=teste&recaptcha_response=03AGddagsdgja" -X POST http://localhost:8080/validate
```