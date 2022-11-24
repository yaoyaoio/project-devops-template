FROM golang:alpine as builder
LABEL maintainer="YAOYAO<i@yaoyao.io>"
ENV GOPROXY="https://goproxy.io"
RUN sed -i 's/dl-cdn\.alpinelinux\.org/mirrors\.aliyun\.com/g' /etc/apk/repositories
RUN apk add --no-cache make
WORKDIR /usr/src/app
COPY . /usr/src/app
RUN make build

FROM alpine
LABEL maintainer="YAOYAO<i@yaoyao.io>"
COPY --from=builder /usr/src/app/target/go-sample /main
ENTRYPOINT ["/main"]
EXPOSE 8080



