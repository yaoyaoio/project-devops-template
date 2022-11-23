FROM rust as builder
LABEL maintainer="YaoYao<i@yaoyao.io>"
WORKDIR /usr/local/app
COPY . /usr/local/app/
RUN cargo build --release

FROM alpine
LABEL maintainer="YaoYao<i@yaoyao.io>"
WORKDIR /usr/local/app
COPY --from=builder /usr/local/app/target/release/rust-sample /usr/local/app/rust-sample
ENTRYPOINT ["/usr/local/app/rust-sample"]
EXPOSE 8080