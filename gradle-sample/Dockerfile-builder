FROM gradle:7.5.1-jdk8 as builder

RUN cp /etc/apk/repositories /etc/apk/repositories.bak
RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g' /etc/apk/repositories

# Install toolsl
RUN apk update && apk add curl bash libstdc++ && rm -rf /var/cache/apk/*

# Set tmp
VOLUME /tmp

# Workdir
WORKDIR /usr/local/app/

# Copy project to docker
COPY . /usr/local/app/
# Build faster
RUN mkdir ~/.gradle && touch ~/.gradle/gradle.properties && echo "org.gradle.daemon=true" >> ~/.gradle/gradle.properties
# Compile
RUN ./gradlew clean build

FROM openjdk:8-jdk-alpine
WORKDIR /usr/local/app/
COPY --from=builder /usr/local/app/build/libs/*.jar /usr/local/app/app.jar
ENTRYPOINT ["java", "-Duser.timezone=GMT+8 -Xms256m -Xmx2048m","-jar", "app.jar"]