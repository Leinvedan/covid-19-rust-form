const WAIT_INSERT_CAPTCHA = 1000 //ms;


function addScriptToCTA() {
  document.getElementById("cta-button").addEventListener('click', () => {
    scrollTo(document.getElementById("covid-form-container"));
  });
}

function afterPageLoad() {
  addScriptToCTA();
  addCepHandler();
  setTimeout(() => { loadCaptchaScript() }, WAIT_INSERT_CAPTCHA);
}

window.onload = afterPageLoad;
