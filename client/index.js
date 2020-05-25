const express = require('express');
const app = express();
const PORT = 5500;
const HOST = '0.0.0.0';

app.use(express.static('dist'));
app.listen(PORT, HOST);

console.log(`Running at ${HOST}:${PORT}`);