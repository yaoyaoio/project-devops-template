FROM openjdk:8-jdk-alpine
WORKDIR /usr/local/app/
COPY build/libs/*.jar /usr/local/app/app.jar
ENTRYPOINT ["java", "-Duser.timezone=GMT+8 -Xms256m -Xmx2048m","-jar", "app.jar"]
EXPOSE 8080