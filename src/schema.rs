table! {
  pessoa (id) {
      id -> Unsigned<Integer>,
      cep -> Text,
      logradouro -> Text,
      bairro -> Text,
      cidade -> Text,
      estado -> Text,
      trabSaude -> Nullable<Text>,
      idade -> Unsigned<Integer>,
      sexo -> Nullable<Text>,
      sintomas -> Unsigned<Integer>,
      dataSintoma -> Nullable<Text>,
      atendimentoMes -> Text,
      parenteConfirmado -> Text,
      casoSuspeito -> Text,
      casoConfirmado -> Text,
      recaptcha_response -> Text,
  }
}