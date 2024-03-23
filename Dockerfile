FROM docker.io/rust:1.67

COPY . . 

EXPOSE 8080

RUN cargo b

CMD ["cargo", "r", "--bin", "client"]
