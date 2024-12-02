FROM node:20-alpine

WORKDIR /front

COPY ./package.json /front/package.json
COPY ./package-lock.json /front/package-lock.json

RUN npm install

COPY ./ /front/