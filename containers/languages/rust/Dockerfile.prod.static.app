FROM rust:1.70 as BUILDER

ARG target=x86_64-unknown-linux-gnu

COPY src/  /app/src/
COPY Cargo.toml /app/Cargo.toml
RUN cd /app && \
    export RUSTFLAGS='-C target-feature=+crt-static' && \
    cargo build \
        --release \
        --target $target && \
    mv target/$target/release/application /opt/application && \
    strip --strip-all /opt/application


FROM gcr.io/distroless/static-debian10:nonroot

COPY --from=BUILDER --chown=nonroot:nonroot /opt/application /home/app/application
USER nonroot
WORKDIR /home/app/bin
ENTRYPOINT [ "/home/app/application" ]
CMD [ "" ]
