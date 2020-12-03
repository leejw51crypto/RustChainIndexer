/// Params defines the set of params for the distribution module.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub community_tax: std::string::String,
    #[prost(string, tag = "2")]
    pub base_proposer_reward: std::string::String,
    #[prost(string, tag = "3")]
    pub bonus_proposer_reward: std::string::String,
    #[prost(bool, tag = "4")]
    pub withdraw_addr_enabled: bool,
}
/// ValidatorHistoricalRewards represents historical rewards for a validator.
/// Height is implicit within the store key.
/// Cumulative reward ratio is the sum from the zeroeth period
/// until this period of rewards / tokens, per the spec.
/// The reference count indicates the number of objects
/// which might need to reference this historical entry at any point.
/// ReferenceCount =
///    number of outstanding delegations which ended the associated period (and
///    might need to read that record)
///  + number of slashes which ended the associated period (and might need to
///  read that record)
///  + one per validator for the zeroeth period, set on initialization
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewards {
    #[prost(message, repeated, tag = "1")]
    pub cumulative_reward_ratio: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint32, tag = "2")]
    pub reference_count: u32,
}
/// ValidatorCurrentRewards represents current rewards and current
/// period for a validator kept as a running counter and incremented
/// each block as long as the validator's tokens remain constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint64, tag = "2")]
    pub period: u64,
}
/// ValidatorAccumulatedCommission represents accumulated commission
/// for a validator kept as a running counter, can be withdrawn at any time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommission {
    #[prost(message, repeated, tag = "1")]
    pub commission: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorOutstandingRewards represents outstanding (un-withdrawn) rewards
/// for a validator inexpensive to track, allows simple sanity checks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorSlashEvent represents a validator slash event.
/// Height is implicit within the store key.
/// This is needed to calculate appropriate amount of staking tokens
/// for delegations which are withdrawn after a slash has occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvent {
    #[prost(uint64, tag = "1")]
    pub validator_period: u64,
    #[prost(string, tag = "2")]
    pub fraction: std::string::String,
}
/// ValidatorSlashEvents is a collection of ValidatorSlashEvent messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvents {
    #[prost(message, repeated, tag = "1")]
    pub validator_slash_events: ::std::vec::Vec<ValidatorSlashEvent>,
}
/// FeePool is the global fee pool for distribution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeePool {
    #[prost(message, repeated, tag = "1")]
    pub community_pool: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// CommunityPoolSpendProposal details a proposal for use of community funds,
/// together with how many coins are proposed to be spent, and to which
/// recipient account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    #[prost(string, tag = "3")]
    pub recipient: std::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// DelegatorStartingInfo represents the starting info for a delegator reward
/// period. It tracks the previous validator period, the delegation's amount of
/// staking token, and the creation height (to check later on if any slashes have
/// occurred). NOTE: Even though validators are slashed to whole staking tokens,
/// the delegators within the validator may be left with less than a full token,
/// thus sdk.Dec is used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfo {
    #[prost(uint64, tag = "1")]
    pub previous_period: u64,
    #[prost(string, tag = "2")]
    pub stake: std::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
}
/// DelegationDelegatorReward represents the properties
/// of a delegator's delegation reward.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationDelegatorReward {
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    #[prost(message, repeated, tag = "2")]
    pub reward: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// CommunityPoolSpendProposalWithDeposit defines a CommunityPoolSpendProposal
/// with a deposit
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    #[prost(string, tag = "3")]
    pub recipient: std::string::String,
    #[prost(string, tag = "4")]
    pub amount: std::string::String,
    #[prost(string, tag = "5")]
    pub deposit: std::string::String,
}
/// DelegatorWithdrawInfo is the address for where distributions rewards are
/// withdrawn to by default this struct is only used at genesis to feed in
/// default withdraw addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorWithdrawInfo {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
    /// withdraw_address is the address to withdraw the delegation rewards to.
    #[prost(string, tag = "2")]
    pub withdraw_address: std::string::String,
}
/// ValidatorOutstandingRewardsRecord is used for import/export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    /// outstanding_rewards represents the oustanding rewards of a validator.
    #[prost(message, repeated, tag = "2")]
    pub outstanding_rewards: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorAccumulatedCommissionRecord is used for import / export via genesis
/// json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommissionRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    /// accumulated is the accumulated commission of a validator.
    #[prost(message, optional, tag = "2")]
    pub accumulated: ::std::option::Option<ValidatorAccumulatedCommission>,
}
/// ValidatorHistoricalRewardsRecord is used for import / export via genesis
/// json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    /// period defines the period the historical rewards apply to.
    #[prost(uint64, tag = "2")]
    pub period: u64,
    /// rewards defines the historical rewards of a validator.
    #[prost(message, optional, tag = "3")]
    pub rewards: ::std::option::Option<ValidatorHistoricalRewards>,
}
/// ValidatorCurrentRewardsRecord is used for import / export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    /// rewards defines the current rewards of a validator.
    #[prost(message, optional, tag = "2")]
    pub rewards: ::std::option::Option<ValidatorCurrentRewards>,
}
/// DelegatorStartingInfoRecord used for import / export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfoRecord {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
    /// validator_address is the address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: std::string::String,
    /// starting_info defines the starting info of a delegator.
    #[prost(message, optional, tag = "3")]
    pub starting_info: ::std::option::Option<DelegatorStartingInfo>,
}
/// ValidatorSlashEventRecord is used for import / export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEventRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    /// height defines the block height at which the slash event occured.
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// period is the period of the slash event.
    #[prost(uint64, tag = "3")]
    pub period: u64,
    /// validator_slash_event describes the slash event.
    #[prost(message, optional, tag = "4")]
    pub validator_slash_event: ::std::option::Option<ValidatorSlashEvent>,
}
/// GenesisState defines the distribution module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::std::option::Option<Params>,
    /// fee_pool defines the fee pool at genesis.
    #[prost(message, optional, tag = "2")]
    pub fee_pool: ::std::option::Option<FeePool>,
    /// fee_pool defines the delegator withdraw infos at genesis.
    #[prost(message, repeated, tag = "3")]
    pub delegator_withdraw_infos: ::std::vec::Vec<DelegatorWithdrawInfo>,
    /// fee_pool defines the previous proposer at genesis.
    #[prost(string, tag = "4")]
    pub previous_proposer: std::string::String,
    /// fee_pool defines the outstanding rewards of all validators at genesis.
    #[prost(message, repeated, tag = "5")]
    pub outstanding_rewards: ::std::vec::Vec<ValidatorOutstandingRewardsRecord>,
    /// fee_pool defines the accumulated commisions of all validators at genesis.
    #[prost(message, repeated, tag = "6")]
    pub validator_accumulated_commissions: ::std::vec::Vec<ValidatorAccumulatedCommissionRecord>,
    /// fee_pool defines the historical rewards of all validators at genesis.
    #[prost(message, repeated, tag = "7")]
    pub validator_historical_rewards: ::std::vec::Vec<ValidatorHistoricalRewardsRecord>,
    /// fee_pool defines the current rewards of all validators at genesis.
    #[prost(message, repeated, tag = "8")]
    pub validator_current_rewards: ::std::vec::Vec<ValidatorCurrentRewardsRecord>,
    /// fee_pool defines the delegator starting infos at genesis.
    #[prost(message, repeated, tag = "9")]
    pub delegator_starting_infos: ::std::vec::Vec<DelegatorStartingInfoRecord>,
    /// fee_pool defines the validator slash events at genesis.
    #[prost(message, repeated, tag = "10")]
    pub validator_slash_events: ::std::vec::Vec<ValidatorSlashEventRecord>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::std::option::Option<Params>,
}
/// QueryValidatorOutstandingRewardsRequest is the request type for the
/// Query/ValidatorOutstandingRewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorOutstandingRewardsRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
}
/// QueryValidatorOutstandingRewardsResponse is the response type for the
/// Query/ValidatorOutstandingRewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorOutstandingRewardsResponse {
    #[prost(message, optional, tag = "1")]
    pub rewards: ::std::option::Option<ValidatorOutstandingRewards>,
}
/// QueryValidatorCommissionRequest is the request type for the
/// Query/ValidatorCommission RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorCommissionRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
}
/// QueryValidatorCommissionResponse is the response type for the
/// Query/ValidatorCommission RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorCommissionResponse {
    /// commission defines the commision the validator received.
    #[prost(message, optional, tag = "1")]
    pub commission: ::std::option::Option<ValidatorAccumulatedCommission>,
}
/// QueryValidatorSlashesRequest is the request type for the
/// Query/ValidatorSlashes RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSlashesRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
    /// starting_height defines the optional starting height to query the slashes.
    #[prost(uint64, tag = "2")]
    pub starting_height: u64,
    /// starting_height defines the optional ending height to query the slashes.
    #[prost(uint64, tag = "3")]
    pub ending_height: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorSlashesResponse is the response type for the
/// Query/ValidatorSlashes RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSlashesResponse {
    /// slashes defines the slashes the validator received.
    #[prost(message, repeated, tag = "1")]
    pub slashes: ::std::vec::Vec<ValidatorSlashEvent>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::std::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegationRewardsRequest is the request type for the
/// Query/DelegationRewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRewardsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_address: std::string::String,
}
/// QueryDelegationRewardsResponse is the response type for the
/// Query/DelegationRewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRewardsResponse {
    /// rewards defines the rewards accrued by a delegation.
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryDelegationTotalRewardsRequest is the request type for the
/// Query/DelegationTotalRewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationTotalRewardsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
}
/// QueryDelegationTotalRewardsResponse is the response type for the
/// Query/DelegationTotalRewards RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationTotalRewardsResponse {
    /// rewards defines all the rewards accrued by a delegator.
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::std::vec::Vec<DelegationDelegatorReward>,
    /// total defines the sum of all the rewards.
    #[prost(message, repeated, tag = "2")]
    pub total: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryDelegatorValidatorsRequest is the request type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
}
/// QueryDelegatorValidatorsResponse is the response type for the
/// Query/DelegatorValidators RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the validators a delegator is delegating for.
    #[prost(string, repeated, tag = "1")]
    pub validators: ::std::vec::Vec<std::string::String>,
}
/// QueryDelegatorWithdrawAddressRequest is the request type for the
/// Query/DelegatorWithdrawAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorWithdrawAddressRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
}
/// QueryDelegatorWithdrawAddressResponse is the response type for the
/// Query/DelegatorWithdrawAddress RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorWithdrawAddressResponse {
    /// withdraw_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub withdraw_address: std::string::String,
}
/// QueryCommunityPoolRequest is the request type for the Query/CommunityPool RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolRequest {}
/// QueryCommunityPoolResponse is the response type for the Query/CommunityPool
/// RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolResponse {
    /// pool defines community pool's coins.
    #[prost(message, repeated, tag = "1")]
    pub pool: ::std::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
#[doc = r" Generated client implementations."]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Query defines the gRPC querier service for distribution module."]
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
        #[doc = " Params queries params of the distribution module."]
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.distribution.v1beta1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ValidatorOutstandingRewards queries rewards of a validator address."]
        pub async fn validator_outstanding_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorOutstandingRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorOutstandingRewardsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/ValidatorOutstandingRewards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ValidatorCommission queries accumulated commission for a validator."]
        pub async fn validator_commission(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorCommissionRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorCommissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/ValidatorCommission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ValidatorSlashes queries slash events of a validator."]
        pub async fn validator_slashes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorSlashesRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorSlashesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/ValidatorSlashes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DelegationRewards queries the total rewards accrued by a delegation."]
        pub async fn delegation_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationRewardsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/DelegationRewards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DelegationTotalRewards queries the total rewards accrued by a each"]
        #[doc = " validator."]
        pub async fn delegation_total_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationTotalRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationTotalRewardsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/DelegationTotalRewards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DelegatorValidators queries the validators of a delegator."]
        pub async fn delegator_validators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorValidatorsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegatorValidatorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/DelegatorValidators",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DelegatorWithdrawAddress queries withdraw address of a delegator."]
        pub async fn delegator_withdraw_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorWithdrawAddressRequest>,
        ) -> Result<tonic::Response<super::QueryDelegatorWithdrawAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " CommunityPool queries the community pool coins."]
        pub async fn community_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCommunityPoolRequest>,
        ) -> Result<tonic::Response<super::QueryCommunityPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/CommunityPool",
            );
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
        #[doc = " Params queries params of the distribution module."]
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        #[doc = " ValidatorOutstandingRewards queries rewards of a validator address."]
        async fn validator_outstanding_rewards(
            &self,
            request: tonic::Request<super::QueryValidatorOutstandingRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorOutstandingRewardsResponse>, tonic::Status>;
        #[doc = " ValidatorCommission queries accumulated commission for a validator."]
        async fn validator_commission(
            &self,
            request: tonic::Request<super::QueryValidatorCommissionRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorCommissionResponse>, tonic::Status>;
        #[doc = " ValidatorSlashes queries slash events of a validator."]
        async fn validator_slashes(
            &self,
            request: tonic::Request<super::QueryValidatorSlashesRequest>,
        ) -> Result<tonic::Response<super::QueryValidatorSlashesResponse>, tonic::Status>;
        #[doc = " DelegationRewards queries the total rewards accrued by a delegation."]
        async fn delegation_rewards(
            &self,
            request: tonic::Request<super::QueryDelegationRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationRewardsResponse>, tonic::Status>;
        #[doc = " DelegationTotalRewards queries the total rewards accrued by a each"]
        #[doc = " validator."]
        async fn delegation_total_rewards(
            &self,
            request: tonic::Request<super::QueryDelegationTotalRewardsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegationTotalRewardsResponse>, tonic::Status>;
        #[doc = " DelegatorValidators queries the validators of a delegator."]
        async fn delegator_validators(
            &self,
            request: tonic::Request<super::QueryDelegatorValidatorsRequest>,
        ) -> Result<tonic::Response<super::QueryDelegatorValidatorsResponse>, tonic::Status>;
        #[doc = " DelegatorWithdrawAddress queries withdraw address of a delegator."]
        async fn delegator_withdraw_address(
            &self,
            request: tonic::Request<super::QueryDelegatorWithdrawAddressRequest>,
        ) -> Result<tonic::Response<super::QueryDelegatorWithdrawAddressResponse>, tonic::Status>;
        #[doc = " CommunityPool queries the community pool coins."]
        async fn community_pool(
            &self,
            request: tonic::Request<super::QueryCommunityPoolRequest>,
        ) -> Result<tonic::Response<super::QueryCommunityPoolResponse>, tonic::Status>;
    }
    #[doc = " Query defines the gRPC querier service for distribution module."]
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
                "/cosmos.distribution.v1beta1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/ValidatorOutstandingRewards" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorOutstandingRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryValidatorOutstandingRewardsRequest>
                        for ValidatorOutstandingRewardsSvc<T>
                    {
                        type Response = super::QueryValidatorOutstandingRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorOutstandingRewardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).validator_outstanding_rewards(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ValidatorOutstandingRewardsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/ValidatorCommission" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorCommissionSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryValidatorCommissionRequest>
                        for ValidatorCommissionSvc<T>
                    {
                        type Response = super::QueryValidatorCommissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorCommissionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).validator_commission(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ValidatorCommissionSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/ValidatorSlashes" => {
                    #[allow(non_camel_case_types)]
                    struct ValidatorSlashesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryValidatorSlashesRequest>
                        for ValidatorSlashesSvc<T>
                    {
                        type Response = super::QueryValidatorSlashesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryValidatorSlashesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).validator_slashes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ValidatorSlashesSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegationRewards" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDelegationRewardsRequest>
                        for DelegationRewardsSvc<T>
                    {
                        type Response = super::QueryDelegationRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegationRewardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegation_rewards(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DelegationRewardsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegationTotalRewards" => {
                    #[allow(non_camel_case_types)]
                    struct DelegationTotalRewardsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDelegationTotalRewardsRequest>
                        for DelegationTotalRewardsSvc<T>
                    {
                        type Response = super::QueryDelegationTotalRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegationTotalRewardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delegation_total_rewards(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DelegationTotalRewardsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegatorValidators" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorValidatorsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDelegatorValidatorsRequest>
                        for DelegatorValidatorsSvc<T>
                    {
                        type Response = super::QueryDelegatorValidatorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegatorValidatorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delegator_validators(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DelegatorValidatorsSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress" => {
                    #[allow(non_camel_case_types)]
                    struct DelegatorWithdrawAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryDelegatorWithdrawAddressRequest>
                        for DelegatorWithdrawAddressSvc<T>
                    {
                        type Response = super::QueryDelegatorWithdrawAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDelegatorWithdrawAddressRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delegator_withdraw_address(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DelegatorWithdrawAddressSvc(inner);
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
                "/cosmos.distribution.v1beta1.Query/CommunityPool" => {
                    #[allow(non_camel_case_types)]
                    struct CommunityPoolSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryCommunityPoolRequest>
                        for CommunityPoolSvc<T>
                    {
                        type Response = super::QueryCommunityPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryCommunityPoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).community_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CommunityPoolSvc(inner);
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
        const NAME: &'static str = "cosmos.distribution.v1beta1.Query";
    }
}
/// MsgSetWithdrawAddress sets the withdraw address for
/// a delegator (or validator self-delegation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetWithdrawAddress {
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag = "2")]
    pub withdraw_address: std::string::String,
}
/// MsgSetWithdrawAddressResponse defines the Msg/SetWithdrawAddress response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetWithdrawAddressResponse {}
/// MsgWithdrawDelegatorReward represents delegation withdrawal to a delegator
/// from a single validator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawDelegatorReward {
    #[prost(string, tag = "1")]
    pub delegator_address: std::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: std::string::String,
}
/// MsgWithdrawDelegatorRewardResponse defines the Msg/WithdrawDelegatorReward response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawDelegatorRewardResponse {}
/// MsgWithdrawValidatorCommission withdraws the full commission to the validator
/// address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawValidatorCommission {
    #[prost(string, tag = "1")]
    pub validator_address: std::string::String,
}
/// MsgWithdrawValidatorCommissionResponse defines the Msg/WithdrawValidatorCommission response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawValidatorCommissionResponse {}
/// MsgFundCommunityPool allows an account to directly
/// fund the community pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPool {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::std::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub depositor: std::string::String,
}
/// MsgFundCommunityPoolResponse defines the Msg/FundCommunityPool response type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPoolResponse {}
#[doc = r" Generated client implementations."]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Msg defines the distribution Msg service."]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        #[doc = " SetWithdrawAddress defines a method to change the withdraw address "]
        #[doc = " for a delegator (or validator self-delegation)."]
        pub async fn set_withdraw_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetWithdrawAddress>,
        ) -> Result<tonic::Response<super::MsgSetWithdrawAddressResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/SetWithdrawAddress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " WithdrawDelegatorReward defines a method to withdraw rewards of delegator"]
        #[doc = " from a single validator."]
        pub async fn withdraw_delegator_reward(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawDelegatorReward>,
        ) -> Result<tonic::Response<super::MsgWithdrawDelegatorRewardResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/WithdrawDelegatorReward",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " WithdrawValidatorCommission defines a method to withdraw the "]
        #[doc = " full commission to the validator address."]
        pub async fn withdraw_validator_commission(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawValidatorCommission>,
        ) -> Result<tonic::Response<super::MsgWithdrawValidatorCommissionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/WithdrawValidatorCommission",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " FundCommunityPool defines a method to allow an account to directly"]
        #[doc = " fund the community pool."]
        pub async fn fund_community_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFundCommunityPool>,
        ) -> Result<tonic::Response<super::MsgFundCommunityPoolResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/FundCommunityPool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MsgClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MsgClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MsgClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MsgServer."]
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        #[doc = " SetWithdrawAddress defines a method to change the withdraw address "]
        #[doc = " for a delegator (or validator self-delegation)."]
        async fn set_withdraw_address(
            &self,
            request: tonic::Request<super::MsgSetWithdrawAddress>,
        ) -> Result<tonic::Response<super::MsgSetWithdrawAddressResponse>, tonic::Status>;
        #[doc = " WithdrawDelegatorReward defines a method to withdraw rewards of delegator"]
        #[doc = " from a single validator."]
        async fn withdraw_delegator_reward(
            &self,
            request: tonic::Request<super::MsgWithdrawDelegatorReward>,
        ) -> Result<tonic::Response<super::MsgWithdrawDelegatorRewardResponse>, tonic::Status>;
        #[doc = " WithdrawValidatorCommission defines a method to withdraw the "]
        #[doc = " full commission to the validator address."]
        async fn withdraw_validator_commission(
            &self,
            request: tonic::Request<super::MsgWithdrawValidatorCommission>,
        ) -> Result<tonic::Response<super::MsgWithdrawValidatorCommissionResponse>, tonic::Status>;
        #[doc = " FundCommunityPool defines a method to allow an account to directly"]
        #[doc = " fund the community pool."]
        async fn fund_community_pool(
            &self,
            request: tonic::Request<super::MsgFundCommunityPool>,
        ) -> Result<tonic::Response<super::MsgFundCommunityPoolResponse>, tonic::Status>;
    }
    #[doc = " Msg defines the distribution Msg service."]
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Msg> MsgServer<T> {
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
    impl<T, B> Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
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
                "/cosmos.distribution.v1beta1.Msg/SetWithdrawAddress" => {
                    #[allow(non_camel_case_types)]
                    struct SetWithdrawAddressSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetWithdrawAddress>
                        for SetWithdrawAddressSvc<T>
                    {
                        type Response = super::MsgSetWithdrawAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetWithdrawAddress>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_withdraw_address(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetWithdrawAddressSvc(inner);
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawDelegatorReward" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawDelegatorRewardSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawDelegatorReward>
                        for WithdrawDelegatorRewardSvc<T>
                    {
                        type Response = super::MsgWithdrawDelegatorRewardResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawDelegatorReward>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).withdraw_delegator_reward(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = WithdrawDelegatorRewardSvc(inner);
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawValidatorCommission" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawValidatorCommissionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawValidatorCommission>
                        for WithdrawValidatorCommissionSvc<T>
                    {
                        type Response = super::MsgWithdrawValidatorCommissionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawValidatorCommission>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).withdraw_validator_commission(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = WithdrawValidatorCommissionSvc(inner);
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
                "/cosmos.distribution.v1beta1.Msg/FundCommunityPool" => {
                    #[allow(non_camel_case_types)]
                    struct FundCommunityPoolSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFundCommunityPool> for FundCommunityPoolSvc<T> {
                        type Response = super::MsgFundCommunityPoolResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFundCommunityPool>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fund_community_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FundCommunityPoolSvc(inner);
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
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::transport::NamedService for MsgServer<T> {
        const NAME: &'static str = "cosmos.distribution.v1beta1.Msg";
    }
}
