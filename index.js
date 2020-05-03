const express = require('express');
const app = express();
const path = require('path');
const router = express.Router();

const PORT = 5500;

app.use(express.static('dist'));
app.listen(process.env.port || PORT);

console.log(`Running at Port ${PORT}`);