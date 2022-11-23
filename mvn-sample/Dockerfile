FROM openjdk:8-jdk-alpine
LABEL maintainer="YAOYAO<i@yaoyao.io>"
COPY target/*.jar /usr/local/app/main.jar
ENTRYPOINT ["java","-jar","main.jar"]
EXPOSE 8080