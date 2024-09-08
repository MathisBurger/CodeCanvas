FROM golang:1.22-alpine AS build
WORKDIR /app
COPY . .
RUN go mod tidy
RUN go build -o ./bin/usernator cmd/main.go

FROM alpine:3 AS final
COPY --from=build /app/bin/usernator ./usernator
RUN chmod +x ./usernator
EXPOSE 3000
CMD ./usernator