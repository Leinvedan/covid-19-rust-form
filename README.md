# Form API

Receives a captcha token from frontend, validate on Google Captcha endpoint and returns response.

### Example POST request

```bash
curl -d '{"captcha_token":"ndasofi3ohfna"}' -H "Content-Type: application/json" -X POST http://127.0.0.1:8080/validate
```