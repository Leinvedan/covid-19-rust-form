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


function afterPageLoad() {
  /* Button scroll event handler*/
  document.getElementById("cta-button").addEventListener('click', () => {
    scrollTo(document.getElementById("covid-form-container"));
  });

  /* CEP event handler*/
  const cepElement = document.getElementById("cep");

  cepElement.addEventListener("keyup", function () {
    const enterIsPressed = event.key !== "Enter";
    const isNotValidCEP = cepElement.value.length !== 8;
    if (enterIsPressed) {
      return;
    };
    if (isNotValidCEP) {
      alert("O CEP deve conter 8 dígitos")
      return;
    }

    showSpinner();
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
          alert('CEP não encontrado');
        } else {
          document.getElementById("logradouro").value = cepData.logradouro;
          document.getElementById("bairro").value = cepData.bairro;
          document.getElementById("cidade").value = cepData.localidade;
          document.getElementById("uf").value = cepData.uf;
        }
      })
      .catch(() => {
        hideSpinner();
        alert('Houve um erro na validação do CEP');
      });
  });
}

window.onload = afterPageLoad;
