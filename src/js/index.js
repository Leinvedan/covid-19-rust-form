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

function afterPageLoad() {
  /* Button scroll event handler*/
  document.getElementById("cta-button").addEventListener('click', () => {
    scrollTo(document.getElementById("covid-form-container"));
  });

  /* CEP event handler*/
  const cepElement = document.getElementById("cep");
  cepElement.addEventListener("focusout", function () {
    fetch(`https://viacep.com.br/ws/${cepElement.value}/json/unicode/`, { method: 'GET', mode: 'cors' })
      .then( (response) => {
          if (response.status === 400) {
            console.alert('CEP inválido');
            return;
          }
          else if (response.status !== 200) {
            console.alert('Algo deu errado ao buscar o CEP, por favor tente novamente');
            return;
          }
          return response.json();
      })
      .then((cepData) => {
        document.getElementById("logradouro").value = cepData.logradouro;
        document.getElementById("bairro").value = cepData.bairro;
        document.getElementById("cidade").value = cepData.localidade;
        document.getElementById("uf").value = cepData.uf;
      });
  });
}

window.onload = afterPageLoad;
