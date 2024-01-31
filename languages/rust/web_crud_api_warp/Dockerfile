FROM rust:1.75 as BUILDER

ARG target=x86_64-unknown-linux-gnu
COPY src/  /app/src/
COPY health_checker/ /app/health_checker/
COPY Cargo.toml /app/Cargo.toml
RUN cd /app && \
    export RUSTFLAGS='-C target-feature=+crt-static' && \
    cargo build \
        --release \
        --target $target && \
    mv target/$target/release/application /opt/application && \
    strip --strip-all /opt/application && \
    mv target/$target/release/health_checker /opt/health_checker && \
    strip --strip-all /opt/health_checker


FROM gcr.io/distroless/static-debian10:nonroot

COPY --from=BUILDER --chown=nonroot:nonroot /opt/application /home/app/application
COPY --from=BUILDER --chown=nonroot:nonroot /opt/health_checker /home/app/health_checker
USER nonroot
WORKDIR /home/app/bin
EXPOSE 24316
HEALTHCHECK \
    --start-period=5s \
    --interval=10s \
    --timeout=10s \
    --retries=3 \
    CMD [ "/home/app/health_checker", "http://localhost:24316/ping" ]
ENTRYPOINT [ "/home/app/application", "--port", "24316" ]
