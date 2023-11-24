use crate::proto_types::neutron::dex::{
    QueryAllUserDepositsRequest, QueryAllUserDepositsResponse, QueryAllUserLimitOrdersRequest,
    QueryAllUserLimitOrdersResponse, QueryEstimateMultiHopSwapRequest,
    QueryEstimateMultiHopSwapResponse, QueryEstimatePlaceLimitOrderRequest,
    QueryEstimatePlaceLimitOrderResponse, QueryGetInactiveLimitOrderTrancheRequest,
    QueryGetInactiveLimitOrderTrancheResponse, QueryGetLimitOrderTrancheRequest,
    QueryGetLimitOrderTrancheResponse, QueryGetLimitOrderTrancheUserRequest,
    QueryGetLimitOrderTrancheUserResponse, QueryGetPoolReservesRequest,
    QueryGetPoolReservesResponse, QueryParamsRequest, QueryParamsResponse,
};
use crate::stargate::aux::make_stargate_query;
use cosmwasm_std::{QuerierWrapper, StdResult};

const PARAMS_QUERY_PATH: &str = "/neutron.dex.Query/Params";
const LIMIT_ORDER_TRANCHE_USER_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTrancheUser";
const LIMIT_ORDER_TRANCHE_QUERY_PATH: &str = "/neutron.dex.Query/LimitOrderTranche";
const USER_DEPOSITS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserDepositsAll";
const USER_LIMIT_ORDERS_ALL_QUERY_PATH: &str = "/neutron.dex.Query/UserLimitOrdersAll";
const INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH: &str =
    "/neutron.dex.Query/InactiveLimitOrderTranche";
const POOL_RESERVES_QUERY_PATH: &str = "/neutron.dex.Query/PoolReserves";
const ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH: &str = "/neutron.dex.Query/EstimateMultiHopSwap";
const ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH: &str = "/neutron.dex.Query/EstimatePlaceLimitOrder";

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

pub fn get_limit_order_tranche(
    querier: QuerierWrapper,
    req: QueryGetLimitOrderTrancheRequest,
) -> StdResult<QueryGetLimitOrderTrancheResponse> {
    make_stargate_query(querier, req, LIMIT_ORDER_TRANCHE_QUERY_PATH)
}

pub fn get_user_deposits_all(
    querier: QuerierWrapper,
    req: QueryAllUserDepositsRequest,
) -> StdResult<QueryAllUserDepositsResponse> {
    make_stargate_query(querier, req, USER_DEPOSITS_ALL_QUERY_PATH)
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
