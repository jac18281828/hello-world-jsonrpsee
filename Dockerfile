FROM ghcr.io/jac18281828/rust:latest

ARG PROJECT=rust1
WORKDIR /workspaces/${PROJECT}

USER rust
ENV USER=rust
ENV PATH=/home/${USER}/.cargo/bin:$PATH:/usr/local/go/bin
# source $HOME/.cargo/env

COPY --chown=rust:rust . .

RUN yamlfmt -lint .github/*.yml .github/workflows/*.yml

RUN cargo fmt --check
RUN cargo clippy --all-features --no-deps
RUN cargo test

