// @generated
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn deposit(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeposit>,
        ) -> std::result::Result<tonic::Response<super::MsgDepositResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Msg/Deposit");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Msg", "Deposit"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdrawal(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawal>,
        ) -> std::result::Result<tonic::Response<super::MsgWithdrawalResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Msg/Withdrawal");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Msg", "Withdrawal"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn place_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPlaceLimitOrder>,
        ) -> std::result::Result<tonic::Response<super::MsgPlaceLimitOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Msg/PlaceLimitOrder");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Msg", "PlaceLimitOrder"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn withdraw_filled_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawFilledLimitOrder>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawFilledLimitOrderResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutron.dex.Msg/WithdrawFilledLimitOrder");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Msg",
                "WithdrawFilledLimitOrder",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn cancel_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelLimitOrder>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelLimitOrderResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Msg/CancelLimitOrder");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Msg", "CancelLimitOrder"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn multi_hop_swap(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMultiHopSwap>,
        ) -> std::result::Result<tonic::Response<super::MsgMultiHopSwapResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Msg/MultiHopSwap");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Msg", "MultiHopSwap"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Msg/UpdateParams");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn limit_order_tranche_user(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetLimitOrderTrancheUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetLimitOrderTrancheUserResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutron.dex.Query/LimitOrderTrancheUser");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Query",
                "LimitOrderTrancheUser",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn limit_order_tranche_user_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllLimitOrderTrancheUserRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllLimitOrderTrancheUserResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutron.dex.Query/LimitOrderTrancheUserAll");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Query",
                "LimitOrderTrancheUserAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn limit_order_tranche_user_all_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllUserLimitOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllUserLimitOrdersResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/neutron.dex.Query/LimitOrderTrancheUserAllByAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Query",
                "LimitOrderTrancheUserAllByAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn limit_order_tranche(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetLimitOrderTrancheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetLimitOrderTrancheResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/LimitOrderTranche");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "LimitOrderTranche"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn limit_order_tranche_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllLimitOrderTrancheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllLimitOrderTrancheResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutron.dex.Query/LimitOrderTrancheAll");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "LimitOrderTrancheAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn user_deposits_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllUserDepositsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllUserDepositsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/UserDepositsAll");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "UserDepositsAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn tick_liquidity_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllTickLiquidityRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllTickLiquidityResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/TickLiquidityAll");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "TickLiquidityAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn inactive_limit_order_tranche(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetInactiveLimitOrderTrancheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryGetInactiveLimitOrderTrancheResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/neutron.dex.Query/InactiveLimitOrderTranche",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Query",
                "InactiveLimitOrderTranche",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn inactive_limit_order_tranche_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllInactiveLimitOrderTrancheRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllInactiveLimitOrderTrancheResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/neutron.dex.Query/InactiveLimitOrderTrancheAll",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Query",
                "InactiveLimitOrderTrancheAll",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_reserves_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllPoolReservesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllPoolReservesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/PoolReservesAll");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "PoolReservesAll"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_reserves(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPoolReservesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPoolReservesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/PoolReserves");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "PoolReserves"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_multi_hop_swap(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEstimateMultiHopSwapRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEstimateMultiHopSwapResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutron.dex.Query/EstimateMultiHopSwap");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "EstimateMultiHopSwap"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn estimate_place_limit_order(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEstimatePlaceLimitOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEstimatePlaceLimitOrderResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/neutron.dex.Query/EstimatePlaceLimitOrder");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "neutron.dex.Query",
                "EstimatePlaceLimitOrder",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/Pool");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "Pool"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryPoolByIdRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/PoolByID");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "PoolByID"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGetPoolMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryGetPoolMetadataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/PoolMetadata");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "PoolMetadata"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn pool_metadata_all(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllPoolMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllPoolMetadataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/neutron.dex.Query/PoolMetadataAll");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("neutron.dex.Query", "PoolMetadataAll"));
            self.inner.unary(req, path, codec).await
        }
    }
}
