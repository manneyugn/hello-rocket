FROM rustlang/rust:nightly

WORKDIR /app

RUN apt-get update

RUN apt-get install -y musl-dev

COPY . .

RUN cargo build

EXPOSE 8000

CMD [ "cargo", "run" ]