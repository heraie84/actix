#FROM debian:bullseye-slim
FROM amitgupta4in:bullseye-slim
WORKDIR /app
ADD target/release/todo-actix .
CMD ["/app/todo-actix"]