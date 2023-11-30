use crate::proto_types::neutron::dex::{
    LimitOrderType, MultiHopRoute, QueryAllInactiveLimitOrderTrancheRequest,
    QueryAllInactiveLimitOrderTrancheResponse, QueryAllLimitOrderTrancheRequest,
    QueryAllLimitOrderTrancheResponse, QueryAllLimitOrderTrancheUserRequest,
    QueryAllLimitOrderTrancheUserResponse, QueryAllPoolMetadataRequest,
    QueryAllPoolMetadataResponse, QueryAllPoolReservesRequest, QueryAllPoolReservesResponse,
    QueryAllTickLiquidityRequest, QueryAllTickLiquidityResponse, QueryAllUserDepositsRequest,
    QueryAllUserDepositsResponse, QueryAllUserLimitOrdersRequest, QueryAllUserLimitOrdersResponse,
    QueryEstimateMultiHopSwapRequest, QueryEstimateMultiHopSwapResponse,
    QueryEstimatePlaceLimitOrderRequest, QueryEstimatePlaceLimitOrderResponse,
    QueryGetInactiveLimitOrderTrancheRequest, QueryGetInactiveLimitOrderTrancheResponse,
    QueryGetLimitOrderTrancheRequest, QueryGetLimitOrderTrancheResponse,
    QueryGetLimitOrderTrancheUserRequest, QueryGetLimitOrderTrancheUserResponse,
    QueryGetPoolMetadataRequest, QueryGetPoolMetadataResponse, QueryGetPoolReservesRequest,
    QueryGetPoolReservesResponse, QueryParamsRequest, QueryParamsResponse, QueryPoolByIdRequest,
    QueryPoolRequest, QueryPoolResponse,
};
use crate::stargate::aux::{convert_timestamp, make_stargate_query};
use cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest;
use cosmwasm_std::{QuerierWrapper, StdResult, Timestamp};

const PARAMS_QUERY_PATH: &str = "/neutron.dex.Query/Params";
const LIMIT_ORDER_TRANCHE_USER_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheUser";
const LIMIT_ORDER_TRANCHE_USER_ALL_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheUserAll";
const LIMIT_ORDER_TRANCHE_USER_ALL_BY_ADDRESS_QUERY_PATH: &str =
    "/neutron.dex.Query/LimitOrderTrancheUserAllByAddress";
const LIMIT_ORDER_TRANCHE_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTranche";
const LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheAll";
const USER_DEPOSITS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserDepositsAll";
const TICK_LIQUIDITY_ALL_QUERY_PATH: &str = "/neutron.dex.Query/TickLiquidityAll";
const INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH: &str =
    "/neutron.dex.Query/InactiveLimitOrderTranche";
const INACTIVE_LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH: &str =
    "/neutron.dex.Query/InactiveLimitOrderTrancheAll";
const POOL_RESERVES_ALL_QUERY_PATH: &str = "/neutron.dex.Query/PoolReservesAll";
const POOL_RESERVES_QUERY_PATH: &str = "/neutron.dex.Query/PoolReserves";
const ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH: &str = "/neutron.dex.Query/EstimateMultiHopSwap";
const ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH: &str = "/neutron.dex.Query/EstimatePlaceLimitOrder";
const POOL_QUERY_PATH: &str = "/neutron.dex.Query/Pool";
const POOL_BY_ID_QUERY_PATH: &str = "/neutron.dex.Query/PoolByID";
const POOL_METADATA_QUERY_PATH: &str = "/neutron.dex.Query/PoolMetadata";
const POOL_METADATA_ALL_QUERY_PATH: &str = "/neutron.dex.Query/PoolMetadataAll";

pub fn get_params(querier: QuerierWrapper) -> StdResult<QueryParamsResponse> {
    make_stargate_query(querier, QueryParamsRequest {}, PARAMS_QUERY_PATH)
}

pub fn get_limit_order_tranche_user(
    querier: QuerierWrapper,
    address: String,
    tranche_key: String,
) -> StdResult<QueryGetLimitOrderTrancheUserResponse> {
    make_stargate_query(
        querier,
        QueryGetLimitOrderTrancheUserRequest {
            address,
            tranche_key,
        },
        LIMIT_ORDER_TRANCHE_USER_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche_user_all(
    querier: QuerierWrapper,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllLimitOrderTrancheUserResponse> {
    make_stargate_query(
        querier,
        QueryAllLimitOrderTrancheUserRequest { pagination },
        LIMIT_ORDER_TRANCHE_USER_ALL_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche_user_all_by_address(
    querier: QuerierWrapper,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllUserLimitOrdersResponse> {
    make_stargate_query(
        querier,
        QueryAllUserLimitOrdersRequest {
            address,
            pagination,
        },
        LIMIT_ORDER_TRANCHE_USER_ALL_BY_ADDRESS_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche(
    querier: QuerierWrapper,
    pair_id: String,
    tick_index: i64,
    token_in: String,
    tranche_key: String,
) -> StdResult<QueryGetLimitOrderTrancheResponse> {
    make_stargate_query(
        querier,
        QueryGetLimitOrderTrancheRequest {
            pair_id,
            tick_index,
            token_in,
            tranche_key,
        },
        LIMIT_ORDER_TRANCHE_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche_all(
    querier: QuerierWrapper,
    pair_id: String,
    token_in: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllLimitOrderTrancheResponse> {
    make_stargate_query(
        querier,
        QueryAllLimitOrderTrancheRequest {
            pair_id,
            token_in,
            pagination,
        },
        LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH,
    )
}

pub fn get_user_deposits_all(
    querier: QuerierWrapper,
    address: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllUserDepositsResponse> {
    make_stargate_query(
        querier,
        QueryAllUserDepositsRequest {
            address,
            pagination,
        },
        USER_DEPOSITS_ALL_QUERY_PATH,
    )
}

pub fn get_tick_liquidity_all(
    querier: QuerierWrapper,
    pair_id: String,
    token_in: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllTickLiquidityResponse> {
    make_stargate_query(
        querier,
        QueryAllTickLiquidityRequest {
            pair_id,
            token_in,
            pagination,
        },
        TICK_LIQUIDITY_ALL_QUERY_PATH,
    )
}

pub fn get_inactive_limit_order_tranche(
    querier: QuerierWrapper,
    pair_id: String,
    token_in: String,
    tick_index: i64,
    tranche_key: String,
) -> StdResult<QueryGetInactiveLimitOrderTrancheResponse> {
    make_stargate_query(
        querier,
        QueryGetInactiveLimitOrderTrancheRequest {
            pair_id,
            token_in,
            tick_index,
            tranche_key,
        },
        INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH,
    )
}

pub fn get_inactive_limit_order_tranche_all(
    querier: QuerierWrapper,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllInactiveLimitOrderTrancheResponse> {
    make_stargate_query(
        querier,
        QueryAllInactiveLimitOrderTrancheRequest { pagination },
        INACTIVE_LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH,
    )
}

pub fn get_pool_reserves_all(
    querier: QuerierWrapper,
    pair_id: String,
    token_in: String,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllPoolReservesResponse> {
    make_stargate_query(
        querier,
        QueryAllPoolReservesRequest {
            pair_id,
            token_in,
            pagination,
        },
        POOL_RESERVES_ALL_QUERY_PATH,
    )
}

pub fn get_pool_reserves(
    querier: QuerierWrapper,
    pair_id: String,
    token_in: String,
    tick_index: i64,
    fee: u64,
) -> StdResult<QueryGetPoolReservesResponse> {
    make_stargate_query(
        querier,
        QueryGetPoolReservesRequest {
            pair_id,
            token_in,
            tick_index,
            fee,
        },
        POOL_RESERVES_QUERY_PATH,
    )
}

pub fn get_estimate_multi_hop_swap(
    querier: QuerierWrapper,
    creator: String,
    receiver: String,
    routes: Vec<Vec<String>>,
    amount_in: String,
    exit_limit_price: String,
    pick_best_route: bool,
) -> StdResult<QueryEstimateMultiHopSwapResponse> {
    make_stargate_query(
        querier,
        QueryEstimateMultiHopSwapRequest {
            creator,
            receiver,
            routes: routes
                .into_iter()
                .map(|r| MultiHopRoute { hops: r })
                .collect(),
            amount_in,
            exit_limit_price,
            pick_best_route,
        },
        ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH,
    )
}

pub fn get_estimate_place_limit_order(
    querier: QuerierWrapper,
    creator: String,
    receiver: String,
    token_in: String,
    token_out: String,
    tick_index_in_to_out: i64,
    amount_in: String,
    order_type: LimitOrderType,
    expiration_time: Option<Timestamp>,
    max_amount_out: Option<String>,
) -> StdResult<QueryEstimatePlaceLimitOrderResponse> {
    make_stargate_query(
        querier,
        QueryEstimatePlaceLimitOrderRequest {
            creator,
            receiver,
            token_in,
            token_out,
            tick_index_in_to_out,
            amount_in,
            order_type: i32::from(order_type),
            expiration_time: expiration_time.map(|e| convert_timestamp(e)),
            max_amount_out: max_amount_out.unwrap_or_default(),
        },
        ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH,
    )
}

pub fn get_pool(
    querier: QuerierWrapper,
    pair_id: String,
    tick_index: i64,
    fee: u64,
) -> StdResult<QueryPoolResponse> {
    make_stargate_query(
        querier,
        QueryPoolRequest {
            pair_id,
            tick_index,
            fee,
        },
        POOL_QUERY_PATH,
    )
}

pub fn get_pool_by_id(querier: QuerierWrapper, pool_id: u64) -> StdResult<QueryPoolResponse> {
    make_stargate_query(
        querier,
        QueryPoolByIdRequest { pool_id },
        POOL_BY_ID_QUERY_PATH,
    )
}

pub fn get_pool_metadata(
    querier: QuerierWrapper,
    id: u64,
) -> StdResult<QueryGetPoolMetadataResponse> {
    make_stargate_query(
        querier,
        QueryGetPoolMetadataRequest { id },
        POOL_METADATA_QUERY_PATH,
    )
}

pub fn get_pool_metadata_all(
    querier: QuerierWrapper,
    pagination: Option<PageRequest>,
) -> StdResult<QueryAllPoolMetadataResponse> {
    make_stargate_query(
        querier,
        QueryAllPoolMetadataRequest { pagination },
        POOL_METADATA_ALL_QUERY_PATH,
    )
}
