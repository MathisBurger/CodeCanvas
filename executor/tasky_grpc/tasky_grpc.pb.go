// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.5.1
// - protoc             v5.26.1
// source: tasky.proto

package tasky_grpc

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
	TaskyApi_GetUserGroups_FullMethodName        = "/tasky_grpc.TaskyApi/GetUserGroups"
	TaskyApi_UpdateSolutionStatus_FullMethodName = "/tasky_grpc.TaskyApi/UpdateSolutionStatus"
)

// TaskyApiClient is the client API for TaskyApi service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type TaskyApiClient interface {
	GetUserGroups(ctx context.Context, in *GroupsRequest, opts ...grpc.CallOption) (*GroupsResponse, error)
	UpdateSolutionStatus(ctx context.Context, in *SolutionUpdateStatusRequest, opts ...grpc.CallOption) (*SolutionUpdateStatusResponse, error)
}

type taskyApiClient struct {
	cc grpc.ClientConnInterface
}

func NewTaskyApiClient(cc grpc.ClientConnInterface) TaskyApiClient {
	return &taskyApiClient{cc}
}

func (c *taskyApiClient) GetUserGroups(ctx context.Context, in *GroupsRequest, opts ...grpc.CallOption) (*GroupsResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(GroupsResponse)
	err := c.cc.Invoke(ctx, TaskyApi_GetUserGroups_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *taskyApiClient) UpdateSolutionStatus(ctx context.Context, in *SolutionUpdateStatusRequest, opts ...grpc.CallOption) (*SolutionUpdateStatusResponse, error) {
	cOpts := append([]grpc.CallOption{grpc.StaticMethod()}, opts...)
	out := new(SolutionUpdateStatusResponse)
	err := c.cc.Invoke(ctx, TaskyApi_UpdateSolutionStatus_FullMethodName, in, out, cOpts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// TaskyApiServer is the server API for TaskyApi service.
// All implementations must embed UnimplementedTaskyApiServer
// for forward compatibility.
type TaskyApiServer interface {
	GetUserGroups(context.Context, *GroupsRequest) (*GroupsResponse, error)
	UpdateSolutionStatus(context.Context, *SolutionUpdateStatusRequest) (*SolutionUpdateStatusResponse, error)
	mustEmbedUnimplementedTaskyApiServer()
}

// UnimplementedTaskyApiServer must be embedded to have
// forward compatible implementations.
//
// NOTE: this should be embedded by value instead of pointer to avoid a nil
// pointer dereference when methods are called.
type UnimplementedTaskyApiServer struct{}

func (UnimplementedTaskyApiServer) GetUserGroups(context.Context, *GroupsRequest) (*GroupsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method GetUserGroups not implemented")
}
func (UnimplementedTaskyApiServer) UpdateSolutionStatus(context.Context, *SolutionUpdateStatusRequest) (*SolutionUpdateStatusResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method UpdateSolutionStatus not implemented")
}
func (UnimplementedTaskyApiServer) mustEmbedUnimplementedTaskyApiServer() {}
func (UnimplementedTaskyApiServer) testEmbeddedByValue()                  {}

// UnsafeTaskyApiServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to TaskyApiServer will
// result in compilation errors.
type UnsafeTaskyApiServer interface {
	mustEmbedUnimplementedTaskyApiServer()
}

func RegisterTaskyApiServer(s grpc.ServiceRegistrar, srv TaskyApiServer) {
	// If the following call pancis, it indicates UnimplementedTaskyApiServer was
	// embedded by pointer and is nil.  This will cause panics if an
	// unimplemented method is ever invoked, so we test this at initialization
	// time to prevent it from happening at runtime later due to I/O.
	if t, ok := srv.(interface{ testEmbeddedByValue() }); ok {
		t.testEmbeddedByValue()
	}
	s.RegisterService(&TaskyApi_ServiceDesc, srv)
}

func _TaskyApi_GetUserGroups_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GroupsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TaskyApiServer).GetUserGroups(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: TaskyApi_GetUserGroups_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TaskyApiServer).GetUserGroups(ctx, req.(*GroupsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _TaskyApi_UpdateSolutionStatus_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(SolutionUpdateStatusRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(TaskyApiServer).UpdateSolutionStatus(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: TaskyApi_UpdateSolutionStatus_FullMethodName,
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(TaskyApiServer).UpdateSolutionStatus(ctx, req.(*SolutionUpdateStatusRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// TaskyApi_ServiceDesc is the grpc.ServiceDesc for TaskyApi service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var TaskyApi_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "tasky_grpc.TaskyApi",
	HandlerType: (*TaskyApiServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "GetUserGroups",
			Handler:    _TaskyApi_GetUserGroups_Handler,
		},
		{
			MethodName: "UpdateSolutionStatus",
			Handler:    _TaskyApi_UpdateSolutionStatus_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "tasky.proto",
}
