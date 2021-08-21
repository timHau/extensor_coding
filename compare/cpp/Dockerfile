FROM ubuntu:latest

RUN apt-get -y update && apt-get install -y

RUN apt-get -y install clang

COPY . /usr/src/extensor_coding

WORKDIR /usr/src/extensor_coding

RUN clang++ -std=c++20 *.cpp -o main

CMD ["./main"]

VOLUME /usr/src/extensor_coding