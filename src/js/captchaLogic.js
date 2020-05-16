const CAPTCHA_KEY = "secretKey";
const WAIT_CAPTCHA_OBJECT = 800; // ms



function loadCaptchaScript() {
  loadCaptchaScript = () => {};
  const tag = document.createElement("script");
  tag.src = `https://www.google.com/recaptcha/api.js?render=${CAPTCHA_KEY}`;
  document.getElementsByTagName("head")[0].appendChild(tag);
  waitCaptchaAndEnableSubmit();
}


function waitCaptchaAndEnableSubmit() {
  let tries = 0;
  const _waitAndLoad = function() {
    if (tries < 5) {
      if (window.grecaptcha) {
        let button = document.getElementById("submit-button");
        button.style.display = "inline-block";
      } else {
        setTimeout(() => {waitCaptchaAndEnableSubmit(tries + 1)}, WAIT_CAPTCHA_OBJECT);
      }
    }
  }
  _waitAndLoad();
}


function getCaptchaTokenAndSubmit() {
  let captchaEl = document.getElementById("recaptchaResponse");
  let form = document.getElementById("covid-form");
  window.grecaptcha.ready(function() {
    window.grecaptcha.execute(CAPTCHA_KEY, {action: 'contact'}).then(function(token) {
        captchaEl.value = token;
        form.submit();
    });
  });
}