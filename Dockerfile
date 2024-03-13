FROM messense/rust-musl-cross:x86_64-musl as chef

RUN cargo install cargo-chef
WORKDIR /app


FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/furdb /bin/furdb

ENTRYPOINT ["/bin/furdb"]

EXPOSE 8080
