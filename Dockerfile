FROM rust:latest as builder

COPY ./ ./

RUN cargo install -q worker-build
RUN worker-build --release
RUN mv *.capnp ./build

FROM ghcr.io/clementreiffers/workerd:latest as worker

COPY --from=builder ./build ./

RUN apt-get install nodejs npm -y

RUN npm i workerd

ENTRYPOINT ["./node_modules/.bin/workerd"]

CMD ["serve", "my-config.capnp"]
