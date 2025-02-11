/// ListAllInterfacesRequest is the request type of the ListAllInterfaces RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterfacesRequest {}
/// ListAllInterfacesResponse is the response type of the ListAllInterfaces RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAllInterfacesResponse {
    /// interface_names is an array of all the registered interfaces.
    #[prost(string, repeated, tag = "1")]
    pub interface_names: ::std::vec::Vec<std::string::String>,
}
/// ListImplementationsRequest is the request type of the ListImplementations
/// RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImplementationsRequest {
    /// interface_name defines the interface to query the implementations for.
    #[prost(string, tag = "1")]
    pub interface_name: std::string::String,
}
/// ListImplementationsResponse is the response type of the ListImplementations
/// RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImplementationsResponse {
    #[prost(string, repeated, tag = "1")]
    pub implementation_message_names: ::std::vec::Vec<std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod reflection_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " ReflectionService defines a service for interface reflection."]
    pub struct ReflectionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReflectionServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ReflectionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " ListAllInterfaces lists all the interfaces registered in the interface"]
        #[doc = " registry."]
        pub async fn list_all_interfaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAllInterfacesRequest>,
        ) -> Result<tonic::Response<super::ListAllInterfacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v1beta1.ReflectionService/ListAllInterfaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ListImplementations list all the concrete types that implement a given"]
        #[doc = " interface."]
        pub async fn list_implementations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImplementationsRequest>,
        ) -> Result<tonic::Response<super::ListImplementationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.reflection.v1beta1.ReflectionService/ListImplementations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ReflectionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ReflectionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ReflectionServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod reflection_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ReflectionServiceServer."]
    #[async_trait]
    pub trait ReflectionService: Send + Sync + 'static {
        #[doc = " ListAllInterfaces lists all the interfaces registered in the interface"]
        #[doc = " registry."]
        async fn list_all_interfaces(
            &self,
            request: tonic::Request<super::ListAllInterfacesRequest>,
        ) -> Result<tonic::Response<super::ListAllInterfacesResponse>, tonic::Status>;
        #[doc = " ListImplementations list all the concrete types that implement a given"]
        #[doc = " interface."]
        async fn list_implementations(
            &self,
            request: tonic::Request<super::ListImplementationsRequest>,
        ) -> Result<tonic::Response<super::ListImplementationsResponse>, tonic::Status>;
    }
    #[doc = " ReflectionService defines a service for interface reflection."]
    #[derive(Debug)]
    pub struct ReflectionServiceServer<T: ReflectionService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ReflectionService> ReflectionServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ReflectionServiceServer<T>
    where
        T: ReflectionService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/cosmos.base.reflection.v1beta1.ReflectionService/ListAllInterfaces" => {
                    #[allow(non_camel_case_types)]
                    struct ListAllInterfacesSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::ListAllInterfacesRequest>
                        for ListAllInterfacesSvc<T>
                    {
                        type Response = super::ListAllInterfacesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAllInterfacesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_all_interfaces(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListAllInterfacesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/cosmos.base.reflection.v1beta1.ReflectionService/ListImplementations" => {
                    #[allow(non_camel_case_types)]
                    struct ListImplementationsSvc<T: ReflectionService>(pub Arc<T>);
                    impl<T: ReflectionService>
                        tonic::server::UnaryService<super::ListImplementationsRequest>
                        for ListImplementationsSvc<T>
                    {
                        type Response = super::ListImplementationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListImplementationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_implementations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListImplementationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ReflectionService> Clone for ReflectionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ReflectionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReflectionService> tonic::transport::NamedService for ReflectionServiceServer<T> {
        const NAME: &'static str = "cosmos.base.reflection.v1beta1.ReflectionService";
    }
}
