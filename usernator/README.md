# Usernator

Setup gRPC proto files:

```shell
protoc --proto_path=./ --go_out=api --go_opt=paths=source_relative api.proto
```