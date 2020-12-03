/// Plan specifies information about a planned upgrade and when it should occur.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Sets the name for the upgrade. This name will be used by the upgraded
    /// version of the software to apply any special "on-upgrade" commands during
    /// the first BeginBlock method after the upgrade is applied. It is also used
    /// to detect whether a software version can handle a given upgrade. If no
    /// upgrade handler with this name has been set in the software, it will be
    /// assumed that the software is out-of-date when the upgrade Time or Height is
    /// reached and the software will exit.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The time after which the upgrade must be performed.
    /// Leave set to its zero value to use a pre-defined Height instead.
    #[prost(message, optional, tag = "2")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    /// The height at which the upgrade must be performed.
    /// Only used if Time is not set.
    #[prost(int64, tag = "3")]
    pub height: i64,
    /// Any application specific upgrade info to be included on-chain
    /// such as a git commit that validators could automatically upgrade to
    #[prost(string, tag = "4")]
    pub info: std::string::String,
    /// IBC-enabled chains can opt-in to including the upgraded client state in its upgrade plan
    /// This will make the chain commit to the correct upgraded (self) client state before the upgrade occurs,
    /// so that connecting chains can verify that the new upgraded client is valid by verifying a proof on the
    /// previous version of the chain.
    /// This will allow IBC connections to persist smoothly across planned chain upgrades
    #[prost(message, optional, tag = "5")]
    pub upgraded_client_state: ::std::option::Option<::prost_types::Any>,
}
/// SoftwareUpgradeProposal is a gov Content type for initiating a software
/// upgrade.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareUpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub plan: ::std::option::Option<Plan>,
}
/// CancelSoftwareUpgradeProposal is a gov Content type for cancelling a software
/// upgrade.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSoftwareUpgradeProposal {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub description: std::string::String,
}
/// QueryCurrentPlanRequest is the request type for the Query/CurrentPlan RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentPlanRequest {}
/// QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentPlanResponse {
    /// plan is the current upgrade plan.
    #[prost(message, optional, tag = "1")]
    pub plan: ::std::option::Option<Plan>,
}
/// QueryCurrentPlanRequest is the request type for the Query/AppliedPlan RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppliedPlanRequest {
    /// name is the name of the applied plan to query for.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppliedPlanResponse {
    /// height is the block height at which the plan was applied.
    #[prost(int64, tag = "1")]
    pub height: i64,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC upgrade querier service."]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        #[doc = " CurrentPlan queries the current upgrade plan."]
        pub async fn current_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCurrentPlanRequest>,
        ) -> Result<tonic::Response<super::QueryCurrentPlanResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Query/CurrentPlan");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " AppliedPlan queries a previously applied upgrade plan by its name."]
        pub async fn applied_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAppliedPlanRequest>,
        ) -> Result<tonic::Response<super::QueryAppliedPlanResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Query/AppliedPlan");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QueryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QueryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QueryClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with QueryServer."]
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        #[doc = " CurrentPlan queries the current upgrade plan."]
        async fn current_plan(
            &self,
            request: tonic::Request<super::QueryCurrentPlanRequest>,
        ) -> Result<tonic::Response<super::QueryCurrentPlanResponse>, tonic::Status>;
        #[doc = " AppliedPlan queries a previously applied upgrade plan by its name."]
        async fn applied_plan(
            &self,
            request: tonic::Request<super::QueryAppliedPlanRequest>,
        ) -> Result<tonic::Response<super::QueryAppliedPlanResponse>, tonic::Status>;
    }
    #[doc = " Query defines the gRPC upgrade querier service."]
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Query> QueryServer<T> {
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
    impl<T, B> Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
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
                "/cosmos.upgrade.v1beta1.Query/CurrentPlan" => {
                    #[allow(non_camel_case_types)]
                    struct CurrentPlanSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCurrentPlanRequest> for CurrentPlanSvc<T> {
                        type Response = super::QueryCurrentPlanResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCurrentPlanRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).current_plan(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CurrentPlanSvc(inner);
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
                "/cosmos.upgrade.v1beta1.Query/AppliedPlan" => {
                    #[allow(non_camel_case_types)]
                    struct AppliedPlanSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAppliedPlanRequest> for AppliedPlanSvc<T> {
                        type Response = super::QueryAppliedPlanResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAppliedPlanRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).applied_plan(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AppliedPlanSvc(inner);
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
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::transport::NamedService for QueryServer<T> {
        const NAME: &'static str = "cosmos.upgrade.v1beta1.Query";
    }
}
