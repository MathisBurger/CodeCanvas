FROM golang:1.22-alpine AS build
WORKDIR /app
COPY . .
RUN go mod tidy
RUN go build -o ./bin/executor cmd/main.go
RUN go build -o ./bin/config_setter config/config.go

FROM alpine:3 AS final
COPY --from=build /app/bin/executor ./executor
COPY --from=build /app/bin/config_setter ./config_setter
COPY --from=build /app/sample_config.toml ./sample_config.toml
RUN chmod +x ./executor
RUN chmod +x ./config_setter
EXPOSE 8000
ENV TORK_CONFIG ./config.toml
CMD ./config_setter && ./executor migration ; ./executor run standalone