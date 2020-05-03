function scrollTo(element) {
  window.scroll({
      behavior: 'smooth',
      left: 0,
      top: element.offsetTop
  });
}


function checkRecaptcha() {
  var response = grecaptcha.getResponse();
  if(response.length == 0) { 
    //reCaptcha not verified
    alert("no pass"); 
  }
  else { 
    //reCaptch verified
    alert("pass"); 
  }
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


function afterPageLoad() {
  /* Button scroll event handler*/
  document.getElementById("cta-button").addEventListener('click', () => {
    scrollTo(document.getElementById("covid-form-container"));
  });

  const cepElement = document.getElementById("cep");
  const warnElement = document.getElementById("cep-warning");
  const cepCallback = debounce(() => callCepAPI(cepElement, warnElement), 700);
  document.getElementById("cep").addEventListener('input', () => {
    cepCallback();
  });
}

window.onload = afterPageLoad;
