const CAPTCHA_KEY = "6LeIxAcTAAAAAJcZVRqyHh71UMIEGNQ_MXjiZKhI";
const WAIT_CAPTCHA_OBJECT = 800; // ms
const WAIT_INSERT_CAPTCHA = 1000 //ms;

function scrollTo(element) {
  window.scroll({
      behavior: 'smooth',
      left: 0,
      top: element.offsetTop
  });
}


function showSpinner() {
  document.getElementById("cep-spinner").classList.remove("hide-spinner");
}


function hideSpinner() {
  document.getElementById("cep-spinner").classList.add("hide-spinner");
}


function debounce(callback, wait) {
  let timeout = null;

  return function() {
    const callNow = !timeout;
    const next = () => callback.apply(this, arguments);
    
    clearTimeout(timeout)
    timeout = setTimeout(next, wait);

    if (callNow) {
      next();
    }
  }
}


function callCepAPI(cepElement, warnElement) {
  const isNotValidCEP = cepElement.value.length !== 8;
  if (isNotValidCEP) {
    warnElement.textContent = 'O CEP deve conter 8 dígitos';
    return;
  }

  showSpinner();
  warnElement.textContent = '';
  const myHeaders = new Headers();
  const myInit = {
    headers: myHeaders,
    method: 'GET',
    mode: 'cors'
  }
  fetch(`https://viacep.com.br/ws/${cepElement.value}/json/unicode/`, myInit)
    .then( (response) => {
        hideSpinner();
        return response.json();
    })
    .then((cepData) => {
      if (cepData.erro) {
        warnElement.textContent = 'CEP não encontrado';
      } else {
        document.getElementById("logradouro").value = cepData.logradouro;
        document.getElementById("bairro").value = cepData.bairro;
        document.getElementById("cidade").value = cepData.localidade;
        document.getElementById("uf").value = cepData.uf;
      }
    })
    .catch(() => {
      hideSpinner();
      warnElement.textContent = 'Houve um erro na validação do CEP';
    });
};


function addScriptToCTA() {
  document.getElementById("cta-button").addEventListener('click', () => {
    scrollTo(document.getElementById("covid-form-container"));
  });
}


function addCepHandler() {
  const cepElement = document.getElementById("cep");
  const warnElement = document.getElementById("cep-warning");
  const cepCallback = debounce(() => callCepAPI(cepElement, warnElement), 700);
  document.getElementById("cep").addEventListener('input', () => {
    cepCallback();
  });
}


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
    window.grecaptcha.execute(CAPTCHA_KEY, {action: 'homepage'}).then(function(token) {
        console.log(`UPDATING TOKEN: ${token}`);
        captchaEl.value = token;
        form.submit();
    });
  });
}

function afterPageLoad() {
  addScriptToCTA();
  addCepHandler();
  setTimeout(() => { loadCaptchaScript() }, WAIT_INSERT_CAPTCHA);
}

window.onload = afterPageLoad;
