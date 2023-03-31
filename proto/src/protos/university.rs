//// Represent a university Id on-chain
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversityId {
    #[prost(bytes = "vec", tag = "1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewUniversityParam {
    #[prost(string, tag = "1")]
    pub ipfs_link: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversityCreationResponse {
    #[prost(message, optional, tag = "1")]
    pub university_link: ::core::option::Option<UniversityId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUniversityParam {
    #[prost(message, optional, tag = "1")]
    pub university_id: ::core::option::Option<UniversityId>,
    #[prost(bool, optional, tag = "2")]
    pub only_verified: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversityInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub university_id: ::core::option::Option<UniversityId>,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<UniversityInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UniversityInfo {
    #[prost(string, tag = "1")]
    pub ipfs_link: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub admin: ::core::option::Option<super::primitives::AccountId>,
}
#[doc = r" Generated client implementations."]
pub mod university_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UniversityServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UniversityServiceClient<tonic::transport::Channel> {
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
    impl<T> UniversityServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UniversityServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UniversityServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = "/ register a new university on-chain"]
        pub async fn register_new_university(
            &mut self,
            request: impl tonic::IntoRequest<super::NewUniversityParam>,
        ) -> Result<tonic::Response<super::UniversityCreationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/university.UniversityService/RegisterNewUniversity",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = "/ get a information about single university giving universityId"]
        pub async fn get_university_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUniversityParam>,
        ) -> Result<tonic::Response<super::UniversityInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/university.UniversityService/GetUniversityInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod university_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UniversityServiceServer."]
    #[async_trait]
    pub trait UniversityService: Send + Sync + 'static {
        #[doc = "/ register a new university on-chain"]
        async fn register_new_university(
            &self,
            request: tonic::Request<super::NewUniversityParam>,
        ) -> Result<tonic::Response<super::UniversityCreationResponse>, tonic::Status>;
        #[doc = "/ get a information about single university giving universityId"]
        async fn get_university_info(
            &self,
            request: tonic::Request<super::GetUniversityParam>,
        ) -> Result<tonic::Response<super::UniversityInfoResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct UniversityServiceServer<T: UniversityService> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UniversityService> UniversityServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UniversityServiceServer<T>
    where
        T: UniversityService,
        B: Body + Send + Sync + 'static,
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
                "/university.UniversityService/RegisterNewUniversity" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterNewUniversitySvc<T: UniversityService>(pub Arc<T>);
                    impl<T: UniversityService>
                        tonic::server::UnaryService<super::NewUniversityParam>
                        for RegisterNewUniversitySvc<T>
                    {
                        type Response = super::UniversityCreationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewUniversityParam>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).register_new_university(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterNewUniversitySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/university.UniversityService/GetUniversityInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetUniversityInfoSvc<T: UniversityService>(pub Arc<T>);
                    impl<T: UniversityService>
                        tonic::server::UnaryService<super::GetUniversityParam>
                        for GetUniversityInfoSvc<T>
                    {
                        type Response = super::UniversityInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUniversityParam>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_university_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetUniversityInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: UniversityService> Clone for UniversityServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: UniversityService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UniversityService> tonic::transport::NamedService for UniversityServiceServer<T> {
        const NAME: &'static str = "university.UniversityService";
    }
}
