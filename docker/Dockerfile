FROM debian:testing

RUN ( \
    set -e; \
    apt-get -yq update; \
    apt-get -yq install \
        build-essential \
        curl \
        mdbook \
        plantuml \
        vim-nox \
    ; \
    apt-get clean; \
)

ENV RUSTUP_HOME=/opt/rustup
ENV CARGO_HOME=/opt/cargo

RUN ( \
    set -e; \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y ; \
    chmod 777 /opt/rustup /opt/cargo; \
)

ENV PATH="$PATH:$CARGO_HOME/bin"
