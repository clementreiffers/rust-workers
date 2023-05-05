FROM rust:latest as builder

COPY ./ ./

RUN cargo install -q worker-build
RUN worker-build --release
RUN mv *.capnp ./build

FROM ubuntu:22.04 as worker

RUN apt-get update && apt-get install -y build-essential git clang libc++-dev libc++abi-dev curl gnupg git python3-pip python3-distutils
RUN mkdir worker && cd worker
COPY --from=builder ./build ./

RUN apt-get install nodejs npm -y

RUN npm i workerd

ENTRYPOINT ["./node_modules/.bin/workerd"]

CMD ["serve", "my-config.capnp"]