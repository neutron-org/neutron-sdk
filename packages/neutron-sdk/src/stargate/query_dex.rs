use crate::proto_types::neutron::dex::{
    QueryAllInactiveLimitOrderTrancheRequest, QueryAllInactiveLimitOrderTrancheResponse,
    QueryAllLimitOrderTrancheRequest, QueryAllLimitOrderTrancheResponse,
    QueryAllLimitOrderTrancheUserRequest, QueryAllLimitOrderTrancheUserResponse,
    QueryAllPoolMetadataRequest, QueryAllPoolMetadataResponse, QueryAllPoolReservesRequest,
    QueryAllPoolReservesResponse, QueryAllTickLiquidityRequest, QueryAllTickLiquidityResponse,
    QueryAllUserDepositsRequest, QueryAllUserDepositsResponse, QueryAllUserLimitOrdersRequest,
    QueryAllUserLimitOrdersResponse, QueryEstimateMultiHopSwapRequest,
    QueryEstimateMultiHopSwapResponse, QueryEstimatePlaceLimitOrderRequest,
    QueryEstimatePlaceLimitOrderResponse, QueryGetInactiveLimitOrderTrancheRequest,
    QueryGetInactiveLimitOrderTrancheResponse, QueryGetLimitOrderTrancheRequest,
    QueryGetLimitOrderTrancheResponse, QueryGetLimitOrderTrancheUserRequest,
    QueryGetLimitOrderTrancheUserResponse, QueryGetPoolMetadataRequest,
    QueryGetPoolMetadataResponse, QueryGetPoolReservesRequest, QueryGetPoolReservesResponse,
    QueryParamsRequest, QueryParamsResponse, QueryPoolByIdRequest, QueryPoolRequest,
    QueryPoolResponse,
};
use crate::stargate::aux::make_stargate_query;
use cosmwasm_std::{QuerierWrapper, StdResult};

const PARAMS_QUERY_PATH: &str = "/neutron.dex.Query/Params";
const LIMIT_ORDER_TRANCHE_USER_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheUser";
const LIMIT_ORDER_TRANCHE_USER_ALL_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheUserAll";
const LIMIT_ORDER_TRANCHE_USER_ALL_BY_ADDRESS_QUERY_PATH: &str =
    "/neutron.dex.Query/LimitOrderTrancheUserAllByAddress";
const LIMIT_ORDER_TRANCHE_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTranche";
const LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheAll";
const USER_DEPOSITS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserDepositsAll";
const TICK_LIQUIDITY_ALL_QUERY_PATH: &str = "/neutron.dex.Query/TickLiquidityAll";
const USER_LIMIT_ORDERS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserLimitOrdersAll";
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

pub fn get_params(
    querier: QuerierWrapper,
    req: QueryParamsRequest,
) -> StdResult<QueryParamsResponse> {
    make_stargate_query(querier, req, PARAMS_QUERY_PATH)
}

pub fn get_limit_order_tranche_user(
    querier: QuerierWrapper,
    req: QueryGetLimitOrderTrancheUserRequest,
) -> StdResult<QueryGetLimitOrderTrancheUserResponse> {
    make_stargate_query(querier, req, LIMIT_ORDER_TRANCHE_USER_QUERY_PATH)
}

pub fn get_limit_order_tranche_user_all(
    querier: QuerierWrapper,
    req: QueryAllLimitOrderTrancheUserRequest,
) -> StdResult<QueryAllLimitOrderTrancheUserResponse> {
    make_stargate_query(querier, req, LIMIT_ORDER_TRANCHE_USER_ALL_QUERY_PATH)
}

pub fn get_limit_order_tranche_user_all_by_address(
    querier: QuerierWrapper,
    req: QueryAllUserLimitOrdersRequest,
) -> StdResult<QueryAllUserLimitOrdersResponse> {
    make_stargate_query(
        querier,
        req,
        LIMIT_ORDER_TRANCHE_USER_ALL_BY_ADDRESS_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche(
    querier: QuerierWrapper,
    req: QueryGetLimitOrderTrancheRequest,
) -> StdResult<QueryGetLimitOrderTrancheResponse> {
    make_stargate_query(querier, req, LIMIT_ORDER_TRANCHE_QUERY_PATH)
}

pub fn get_limit_order_tranche_all(
    querier: QuerierWrapper,
    req: QueryAllLimitOrderTrancheRequest,
) -> StdResult<QueryAllLimitOrderTrancheResponse> {
    make_stargate_query(querier, req, LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH)
}

pub fn get_user_deposits_all(
    querier: QuerierWrapper,
    req: QueryAllUserDepositsRequest,
) -> StdResult<QueryAllUserDepositsResponse> {
    make_stargate_query(querier, req, USER_DEPOSITS_ALL_QUERY_PATH)
}

pub fn get_tick_liquidity_all(
    querier: QuerierWrapper,
    req: QueryAllTickLiquidityRequest,
) -> StdResult<QueryAllTickLiquidityResponse> {
    make_stargate_query(querier, req, TICK_LIQUIDITY_ALL_QUERY_PATH)
}

pub fn get_user_limit_orders_all(
    querier: QuerierWrapper,
    req: QueryAllUserLimitOrdersRequest,
) -> StdResult<QueryAllUserLimitOrdersResponse> {
    make_stargate_query(querier, req, USER_LIMIT_ORDERS_ALL_QUERY_PATH)
}

pub fn get_inactive_limit_order_tranche(
    querier: QuerierWrapper,
    req: QueryGetInactiveLimitOrderTrancheRequest,
) -> StdResult<QueryGetInactiveLimitOrderTrancheResponse> {
    make_stargate_query(querier, req, INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH)
}

pub fn get_inactive_limit_order_tranche_all(
    querier: QuerierWrapper,
    req: QueryAllInactiveLimitOrderTrancheRequest,
) -> StdResult<QueryAllInactiveLimitOrderTrancheResponse> {
    make_stargate_query(querier, req, INACTIVE_LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH)
}

pub fn get_pool_reserves_all(
    querier: QuerierWrapper,
    req: QueryAllPoolReservesRequest,
) -> StdResult<QueryAllPoolReservesResponse> {
    make_stargate_query(querier, req, POOL_RESERVES_ALL_QUERY_PATH)
}

pub fn get_pool_reserves(
    querier: QuerierWrapper,
    req: QueryGetPoolReservesRequest,
) -> StdResult<QueryGetPoolReservesResponse> {
    make_stargate_query(querier, req, POOL_RESERVES_QUERY_PATH)
}

pub fn get_estimate_multi_hop_swap(
    querier: QuerierWrapper,
    req: QueryEstimateMultiHopSwapRequest,
) -> StdResult<QueryEstimateMultiHopSwapResponse> {
    make_stargate_query(querier, req, ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH)
}

pub fn get_estimate_place_limit_order(
    querier: QuerierWrapper,
    req: QueryEstimatePlaceLimitOrderRequest,
) -> StdResult<QueryEstimatePlaceLimitOrderResponse> {
    make_stargate_query(querier, req, ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH)
}

pub fn get_pool(querier: QuerierWrapper, req: QueryPoolRequest) -> StdResult<QueryPoolResponse> {
    make_stargate_query(querier, req, POOL_QUERY_PATH)
}

pub fn get_pool_by_id(
    querier: QuerierWrapper,
    req: QueryPoolByIdRequest,
) -> StdResult<QueryPoolResponse> {
    make_stargate_query(querier, req, POOL_BY_ID_QUERY_PATH)
}

pub fn get_pool_metadata(
    querier: QuerierWrapper,
    req: QueryGetPoolMetadataRequest,
) -> StdResult<QueryGetPoolMetadataResponse> {
    make_stargate_query(querier, req, POOL_METADATA_QUERY_PATH)
}

pub fn get_pool_metadata_all(
    querier: QuerierWrapper,
    req: QueryAllPoolMetadataRequest,
) -> StdResult<QueryAllPoolMetadataResponse> {
    make_stargate_query(querier, req, POOL_METADATA_ALL_QUERY_PATH)
}
