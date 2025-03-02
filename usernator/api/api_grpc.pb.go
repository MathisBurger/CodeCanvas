// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.5.1
// - protoc             v5.26.1
// source: api.proto

package api

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.64.0 or later.
const _ = grpc.SupportPackageIsVersion9

const (
	UsernatorApi_GetUser_FullMethodName        = "/api.UsernatorApi/GetUser"
	UsernatorApi_GetUsers_FullMethodName       = "/api.UsernatorApi/GetUsers"
	UsernatorApi_SearchStudents_FullMethodName = "/api.UsernatorApi/SearchStudents"
)

// UsernatorApiClient is the client API for UsernatorApi service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type UsernatorApiClient interface {
	GetUser(ctx context.Context, in *UserRequest, opts ...grpc.CallOption) (*UserResponse, error)
	GetUsers(ctx context.Context, in *UsersRequest, opts ...grpc.CallOption) (*UsersResponse, error)
	SearchStudents(ctx context.Context, in *SearchStudentsRequest, opts ...grpc.CallOption) (*UsersResponse, error)
}

type usernatorApiClient struct {
	cc grpc.ClientConnInterface
}

func NewUsernatorApiClient(cc grpc.ClientConnInterface) UsernatorApiClient {
	return &usernatorApiClient{cc}
}

func (c *usernatorApiClient) GetUser(ctx context.Context, in *UserRequest, opts ...grpc.CallOption) (*UserResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UserResponse)
	err := c.cc.Invoke(ctx, UsernatorApi_GetUser_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *usernatorApiClient) GetUsers(ctx context.Context, in *UsersRequest, opts ...grpc.CallOption) (*UsersResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UsersResponse)
	err := c.cc.Invoke(ctx, UsernatorApi_GetUsers_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *usernatorApiClient) SearchStudents(ctx context.Context, in *SearchStudentsRequest, opts ...grpc.CallOption) (*UsersResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(UsersResponse)
	err := c.cc.Invoke(ctx, UsernatorApi_SearchStudents_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// UsernatorApiServer is the server API for UsernatorApi service.
// All implementations must embed UnimplementedUsernatorApiServer
// for forward compatibility.
type UsernatorApiServer interface {
	GetUser(context.Context, *UserRequest) (*UserResponse, error)
	GetUsers(context.Context, *UsersRequest) (*UsersResponse, error)
	SearchStudents(context.Context, *SearchStudentsRequest) (*UsersResponse, error)
	mustEmbedUnimplementedUsernatorApiServer()
}

// UnimplementedUsernatorApiServer must be embedded to have
// forward compatible implementations.
//
// NOTE: this should be embedded by value instead of pointer to avoid a nil
// pointer dereference when methods are called.
type UnimplementedUsernatorApiServer struct{}

func (UnimplementedUsernatorApiServer) GetUser(context.Context, *UserRequest) (*UserResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetUser not implemented")
}
func (UnimplementedUsernatorApiServer) GetUsers(context.Context, *UsersRequest) (*UsersResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetUsers not implemented")
}
func (UnimplementedUsernatorApiServer) SearchStudents(context.Context, *SearchStudentsRequest) (*UsersResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method SearchStudents not implemented")
}
func (UnimplementedUsernatorApiServer) mustEmbedUnimplementedUsernatorApiServer() {}
func (UnimplementedUsernatorApiServer) testEmbeddedByValue()                      {}

// UnsafeUsernatorApiServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to UsernatorApiServer will
// result in compilation errors.
type UnsafeUsernatorApiServer interface {
	mustEmbedUnimplementedUsernatorApiServer()
}

func RegisterUsernatorApiServer(s grpc.ServiceRegistrar, srv UsernatorApiServer) {
	// If the following call pancis, it indicates UnimplementedUsernatorApiServer was
	// embedded by pointer and is nil.  This will cause panics if an
	// unimplemented method is ever invoked, so we test this at initialization
	// time to prevent it from happening at runtime later due to I/O.
	if t, ok := srv.(interface{ testEmbeddedByValue() }); ok {
		t.testEmbeddedByValue()
	}
	s.RegisterService(&UsernatorApi_ServiceDesc, srv)
}

func _UsernatorApi_GetUser_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UserRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(UsernatorApiServer).GetUser(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: UsernatorApi_GetUser_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(UsernatorApiServer).GetUser(ctx, req.(*UserRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _UsernatorApi_GetUsers_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UsersRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(UsernatorApiServer).GetUsers(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: UsernatorApi_GetUsers_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(UsernatorApiServer).GetUsers(ctx, req.(*UsersRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _UsernatorApi_SearchStudents_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SearchStudentsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(UsernatorApiServer).SearchStudents(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: UsernatorApi_SearchStudents_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(UsernatorApiServer).SearchStudents(ctx, req.(*SearchStudentsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// UsernatorApi_ServiceDesc is the grpc.ServiceDesc for UsernatorApi service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var UsernatorApi_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "api.UsernatorApi",
	HandlerType: (*UsernatorApiServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetUser",
			Handler:    _UsernatorApi_GetUser_Handler,
		},
		{
			MethodName: "GetUsers",
			Handler:    _UsernatorApi_GetUsers_Handler,
		},
		{
			MethodName: "SearchStudents",
			Handler:    _UsernatorApi_SearchStudents_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "api.proto",
}
