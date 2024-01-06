FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    cmake \
    clang \
    git \
    libasio-dev \
    && apt-get clean

RUN mkdir /app
WORKDIR /app
COPY . /app

RUN cmake -B build . && cmake --build build
EXPOSE 8080
ENTRYPOINT ["./build/api"]
