FROM node:18-alpine as builder
LABEL maintainer="YaoYao<i@yaoyao.io>"

WORKDIR /home/node/app
COPY package.json /home/node/app/
RUN npm install
COPY . /home/node/app
RUN npm run build


FROM nginx:alpine
COPY --from=builder /home/node/app/nginx/default.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /home/node/app/dist /usr/share/nginx/html
EXPOSE 80

