use crate::stargate::aux::make_stargate_query;
use crate::stargate::proto_types::neutron::dex::{
    QueryAllInactiveLimitOrderTrancheRequest, QueryAllLimitOrderTrancheRequest,
    QueryAllLimitOrderTrancheUserRequest, QueryAllPoolMetadataRequest, QueryAllPoolReservesRequest,
    QueryAllTickLiquidityRequest, QueryAllUserDepositsRequest, QueryAllUserLimitOrdersRequest,
    QueryEstimateMultiHopSwapRequest, QueryEstimatePlaceLimitOrderRequest,
    QueryGetInactiveLimitOrderTrancheRequest, QueryGetLimitOrderTrancheRequest,
    QueryGetLimitOrderTrancheUserRequest, QueryGetPoolMetadataRequest, QueryGetPoolReservesRequest,
    QueryParamsRequest, QueryPoolByIdRequest, QueryPoolRequest,
};
use crate::stargate::types_dex::{
    AllInactiveLimitOrderTrancheRequest, AllInactiveLimitOrderTrancheResponse,
    AllLimitOrderTrancheRequest, AllLimitOrderTrancheResponse, AllPoolMetadataRequest,
    AllPoolMetadataResponse, AllPoolReservesRequest, AllPoolReservesResponse,
    AllTickLiquidityRequest, AllTickLiquidityResponse, AllUserDepositsRequest,
    AllUserDepositsResponse, AllUserLimitOrdersRequest, AllUserLimitOrdersResponse,
    EstimateMultiHopSwapRequest, EstimateMultiHopSwapResponse, EstimatePlaceLimitOrderRequest,
    EstimatePlaceLimitOrderResponse, GetInactiveLimitOrderTrancheRequest,
    GetInactiveLimitOrderTrancheResponse, GetLimitOrderTrancheRequest,
    GetLimitOrderTrancheResponse, GetPoolMetadataRequest, GetPoolMetadataResponse,
    GetPoolReservesRequest, GetPoolReservesResponse, LimitOrderTrancheUserAllRequest,
    LimitOrderTrancheUserAllRespose, LimitOrderTrancheUserRequest, LimitOrderTrancheUserResponse,
    ParamsRequest, ParamsResponse, PoolByIdRequest, PoolRequest, PoolResponse,
};
use cosmwasm_std::{Deps, StdResult};

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

pub fn get_params(deps: Deps, req: ParamsRequest) -> StdResult<ParamsResponse> {
    make_stargate_query(deps, QueryParamsRequest::from(req), PARAMS_QUERY_PATH)
}

pub fn get_limit_order_tranche_user(
    deps: Deps,
    req: LimitOrderTrancheUserRequest,
) -> StdResult<LimitOrderTrancheUserResponse> {
    make_stargate_query(
        deps,
        QueryGetLimitOrderTrancheUserRequest::from(req),
        LIMIT_ORDER_TRANCHE_USER_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche_user_all(
    deps: Deps,
    req: LimitOrderTrancheUserAllRequest,
) -> StdResult<LimitOrderTrancheUserAllRespose> {
    make_stargate_query(
        deps,
        QueryAllLimitOrderTrancheUserRequest::from(req),
        LIMIT_ORDER_TRANCHE_USER_ALL_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche_user_all_by_address(
    deps: Deps,
    req: AllUserLimitOrdersRequest,
) -> StdResult<AllUserLimitOrdersResponse> {
    make_stargate_query(
        deps,
        QueryAllUserLimitOrdersRequest::from(req),
        LIMIT_ORDER_TRANCHE_USER_ALL_BY_ADDRESS_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche(
    deps: Deps,
    req: GetLimitOrderTrancheRequest,
) -> StdResult<GetLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        QueryGetLimitOrderTrancheRequest::from(req),
        LIMIT_ORDER_TRANCHE_QUERY_PATH,
    )
}

pub fn get_limit_order_tranche_all(
    deps: Deps,
    req: AllLimitOrderTrancheRequest,
) -> StdResult<AllLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        QueryAllLimitOrderTrancheRequest::from(req),
        LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH,
    )
}

pub fn get_user_deposits_all(
    deps: Deps,
    req: AllUserDepositsRequest,
) -> StdResult<AllUserDepositsResponse> {
    make_stargate_query(
        deps,
        QueryAllUserDepositsRequest::from(req),
        USER_DEPOSITS_ALL_QUERY_PATH,
    )
}

pub fn get_tick_liquidity_all(
    deps: Deps,
    req: AllTickLiquidityRequest,
) -> StdResult<AllTickLiquidityResponse> {
    make_stargate_query(
        deps,
        QueryAllTickLiquidityRequest::from(req),
        TICK_LIQUIDITY_ALL_QUERY_PATH,
    )
}

pub fn get_inactive_limit_order_tranche(
    deps: Deps,
    req: GetInactiveLimitOrderTrancheRequest,
) -> StdResult<GetInactiveLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        QueryGetInactiveLimitOrderTrancheRequest::from(req),
        INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH,
    )
}

pub fn get_inactive_limit_order_tranche_all(
    deps: Deps,
    req: AllInactiveLimitOrderTrancheRequest,
) -> StdResult<AllInactiveLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        QueryAllInactiveLimitOrderTrancheRequest::from(req),
        INACTIVE_LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH,
    )
}

pub fn get_pool_reserves_all(
    deps: Deps,
    req: AllPoolReservesRequest,
) -> StdResult<AllPoolReservesResponse> {
    make_stargate_query(
        deps,
        QueryAllPoolReservesRequest::from(req),
        POOL_RESERVES_ALL_QUERY_PATH,
    )
}

pub fn get_pool_reserves(
    deps: Deps,
    req: GetPoolReservesRequest,
) -> StdResult<GetPoolReservesResponse> {
    make_stargate_query(
        deps,
        QueryGetPoolReservesRequest::from(req),
        POOL_RESERVES_QUERY_PATH,
    )
}

pub fn get_estimate_multi_hop_swap(
    deps: Deps,
    req: EstimateMultiHopSwapRequest,
) -> StdResult<EstimateMultiHopSwapResponse> {
    make_stargate_query(
        deps,
        QueryEstimateMultiHopSwapRequest::from(req),
        ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH,
    )
}

pub fn get_estimate_place_limit_order(
    deps: Deps,
    req: EstimatePlaceLimitOrderRequest,
) -> StdResult<EstimatePlaceLimitOrderResponse> {
    make_stargate_query(
        deps,
        QueryEstimatePlaceLimitOrderRequest::from(req),
        ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH,
    )
}

pub fn get_pool(deps: Deps, req: PoolRequest) -> StdResult<PoolResponse> {
    make_stargate_query(deps, QueryPoolRequest::from(req), POOL_QUERY_PATH)
}

pub fn get_pool_by_id(deps: Deps, req: PoolByIdRequest) -> StdResult<PoolResponse> {
    make_stargate_query(deps, QueryPoolByIdRequest::from(req), POOL_BY_ID_QUERY_PATH)
}

pub fn get_pool_metadata(
    deps: Deps,
    req: GetPoolMetadataRequest,
) -> StdResult<GetPoolMetadataResponse> {
    make_stargate_query(
        deps,
        QueryGetPoolMetadataRequest::from(req),
        POOL_METADATA_QUERY_PATH,
    )
}

pub fn get_pool_metadata_all(
    deps: Deps,
    req: AllPoolMetadataRequest,
) -> StdResult<AllPoolMetadataResponse> {
    make_stargate_query(
        deps,
        QueryAllPoolMetadataRequest::from(req),
        POOL_METADATA_ALL_QUERY_PATH,
    )
}
