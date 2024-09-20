package grpc

import (
	"google.golang.org/grpc"
	"log"
	"net"
	"usernator/api"
)

type GrpcServer struct {
	api.UnimplementedUsernatorApiServer
}

func StartGrpcServer() {
	lis, err := net.Listen("tcp", ":3001")
	if err != nil {
		log.Fatalf("failed to listen: %v", err)
	}
	s := grpc.NewServer()
	api.RegisterUsernatorApiServer(s, &GrpcServer{})
	log.Printf("server listening at %v", lis.Addr())
	if err := s.Serve(lis); err != nil {
		log.Fatalf("failed to serve: %v", err)
	}
}
