FROM node:14.3.0-alpine3.10

COPY package*.json ./

RUN npm install

COPY ./src ./src
COPY gulpfile.js index.js ./

RUN npm run build