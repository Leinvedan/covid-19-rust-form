FROM rust:1-stretch as build

# Create empty project
RUN USER=root cargo new --bin covid-survey
WORKDIR /covid-survey

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# used to cache cargo dependencies
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

# Build the source
RUN rm ./target/release/deps/covid_survey*
RUN cargo build --release

# Rust image without cargo
FROM rust:1-slim-stretch

# copy the build artifact from the build stage
COPY --from=build /covid-survey/target/release/covid-survey .

CMD ["./covid-survey"]