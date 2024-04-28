FROM arm64v8/rust as banshee
WORKDIR /app
COPY ../ /app
#RUN cargo build --release
#CMD ["./target/release/calc-engine"]
