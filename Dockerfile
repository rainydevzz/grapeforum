FROM rust:latest

WORKDIR /usr/bin/grapeforum

COPY . .

EXPOSE 8080
EXPOSE 5432
CMD [ "cargo", "run", "--release" ]