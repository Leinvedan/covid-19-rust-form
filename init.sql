CREATE DATABASE IF NOT EXISTS covidForm;
USE covidForm;

CREATE TABLE IF NOT EXISTS pessoa (
    id INT UNSIGNED NOT NULL AUTO_INCREMENT,
    cep TEXT,
    logradouro TEXT NOT NULL,
    bairro TEXT NOT NULL,
    cidade TEXT NOT NULL,
    estado TEXT NOT NULL,
    trabSaude TEXT,
    idade Integer UNSIGNED,
    sexo TEXT NOT NULL,
    sintomas Integer UNSIGNED NOT NULL,
    dataSintoma TEXT,
    atendimentoMes TEXT NOT NULL,
    parenteConfirmado TEXT NOT NULL,
    casoSuspeito TEXT NOT NULL,
    casoConfirmado TEXT NOT NULL,
    PRIMARY KEY (id)
);

INSERT INTO pessoa (
  id, cep, logradouro, bairro, cidade, estado, idade, sexo, sintomas,
  atendimentoMes, parenteConfirmado, casoSuspeito, casoConfirmado)
VALUES (2131, "224369202", "rua x", "austin", "sp", "sp", 22, "M", 3324, "22-02-2020", "y", "n", "nsi");