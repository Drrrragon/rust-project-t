FROM rust:1.80.1-slim-bullseye

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    curl \
    git \
    zsh \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

SHELL ["/bin/zsh", "-c"]

WORKDIR /workspace

CMD [ "zsh" ]