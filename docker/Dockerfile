FROM rust:1.54.0-slim-buster

RUN apt update && apt install -y locales ca-certificates curl libssl-dev pkg-config

RUN localedef -i ja_JP -c -f UTF-8 -A /usr/share/locale/locale.alias ja_JP.UTF-8
ENV LANG ja_JP.UTF-8

RUN rustup component add rustfmt
RUN cargo install cargo-watch
RUN cargo install wasm-bindgen-cli
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

RUN rm -rf /var/lib/apt/lists/* /var/cache/apt/*

WORKDIR /app

CMD ["cargo", "watch", "--ignore", "tmp", "--", \
    "wasm-pack", "build", "--no-typescript", "--target", "web"]
