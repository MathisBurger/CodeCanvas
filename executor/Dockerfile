FROM golang:1.22-alpine AS build
WORKDIR /app
COPY . .
RUN go mod tidy
RUN go build -o ./bin/executor cmd/main.go

FROM alpine:3 AS final
COPY --from=build /app/bin/executor ./executor
COPY --from=build /app/config.toml config.toml
RUN chmod +x ./executor
EXPOSE 8000
ENV TORK_CONFIG ./config.toml
CMD ./executor migration && ./executor run standalone