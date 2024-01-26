use crate::proto_types::neutron::dex::{
    QueryAllInactiveLimitOrderTrancheRequest, QueryAllLimitOrderTrancheRequest,
    QueryAllLimitOrderTrancheUserRequest, QueryAllPoolMetadataRequest, QueryAllPoolReservesRequest,
    QueryAllTickLiquidityRequest, QueryAllUserDepositsRequest, QueryAllUserLimitOrdersRequest,
    QueryEstimateMultiHopSwapRequest, QueryEstimatePlaceLimitOrderRequest,
    QueryGetInactiveLimitOrderTrancheRequest, QueryGetLimitOrderTrancheRequest,
    QueryGetLimitOrderTrancheUserRequest, QueryGetPoolMetadataRequest, QueryGetPoolReservesRequest,
    QueryParamsRequest, QueryPoolByIdRequest, QueryPoolRequest,
};
use crate::stargate::aux::make_stargate_query;
use crate::stargate::dex::types::{
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
    LimitOrderTrancheUserAllResponse, LimitOrderTrancheUserRequest, LimitOrderTrancheUserResponse,
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

/// Queries the parameters of the module.
pub fn get_params(deps: Deps, req: ParamsRequest) -> StdResult<ParamsResponse> {
    make_stargate_query(deps, PARAMS_QUERY_PATH, QueryParamsRequest::from(req))
}

/// Retrieves a `LimitOrderTrancheUser` by user address and tranche key.
pub fn get_limit_order_tranche_user(
    deps: Deps,
    req: LimitOrderTrancheUserRequest,
) -> StdResult<LimitOrderTrancheUserResponse> {
    make_stargate_query(
        deps,
        LIMIT_ORDER_TRANCHE_USER_QUERY_PATH,
        QueryGetLimitOrderTrancheUserRequest::from(req),
    )
}

/// Retrieves a list of `LimitOrderTrancheUser` items.
pub fn get_limit_order_tranche_user_all(
    deps: Deps,
    req: LimitOrderTrancheUserAllRequest,
) -> StdResult<LimitOrderTrancheUserAllResponse> {
    make_stargate_query(
        deps,
        LIMIT_ORDER_TRANCHE_USER_ALL_QUERY_PATH,
        QueryAllLimitOrderTrancheUserRequest::from(req),
    )
}

/// Retrieves a list of `LimitOrderTrancheUser` items by user address.
pub fn get_limit_order_tranche_user_all_by_address(
    deps: Deps,
    req: AllUserLimitOrdersRequest,
) -> StdResult<AllUserLimitOrdersResponse> {
    make_stargate_query(
        deps,
        LIMIT_ORDER_TRANCHE_USER_ALL_BY_ADDRESS_QUERY_PATH,
        QueryAllUserLimitOrdersRequest::from(req),
    )
}

/// Retrieves a `LimitOrderTranche` by a tranche's key (pair_id + token_in + tick_index + tranche_key).
pub fn get_limit_order_tranche(
    deps: Deps,
    req: GetLimitOrderTrancheRequest,
) -> StdResult<GetLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        LIMIT_ORDER_TRANCHE_QUERY_PATH,
        QueryGetLimitOrderTrancheRequest::from(req),
    )
}

/// Retrieves a list of `LimitOrderTranche` items for a given pair_id / token_in combination.
pub fn get_limit_order_tranche_all(
    deps: Deps,
    req: AllLimitOrderTrancheRequest,
) -> StdResult<AllLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH,
        QueryAllLimitOrderTrancheRequest::from(req),
    )
}

/// Retrieves a list of `DepositRecord` items by user address.
pub fn get_user_deposits_all(
    deps: Deps,
    req: AllUserDepositsRequest,
) -> StdResult<AllUserDepositsResponse> {
    make_stargate_query(
        deps,
        USER_DEPOSITS_ALL_QUERY_PATH,
        QueryAllUserDepositsRequest::from(req),
    )
}

/// Retrieves a list of `TickLiquidity` items for a given pair_id / token_in combination.
pub fn get_tick_liquidity_all(
    deps: Deps,
    req: AllTickLiquidityRequest,
) -> StdResult<AllTickLiquidityResponse> {
    make_stargate_query(
        deps,
        TICK_LIQUIDITY_ALL_QUERY_PATH,
        QueryAllTickLiquidityRequest::from(req),
    )
}

/// Retrieves an inactive `LimitOrderTranche` by index.
pub fn get_inactive_limit_order_tranche(
    deps: Deps,
    req: GetInactiveLimitOrderTrancheRequest,
) -> StdResult<GetInactiveLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        INACTIVE_LIMIT_ORDER_TRANCHE_QUERY_PATH,
        QueryGetInactiveLimitOrderTrancheRequest::from(req),
    )
}

/// Retrieves a list of inactive `LimitOrderTranche` items.
pub fn get_inactive_limit_order_tranche_all(
    deps: Deps,
    req: AllInactiveLimitOrderTrancheRequest,
) -> StdResult<AllInactiveLimitOrderTrancheResponse> {
    make_stargate_query(
        deps,
        INACTIVE_LIMIT_ORDER_TRANCHE_ALL_QUERY_PATH,
        QueryAllInactiveLimitOrderTrancheRequest::from(req),
    )
}

/// Retrieves a list of `PoolReserves` items for a given pair_id / token_in combination.
pub fn get_pool_reserves_all(
    deps: Deps,
    req: AllPoolReservesRequest,
) -> StdResult<AllPoolReservesResponse> {
    make_stargate_query(
        deps,
        POOL_RESERVES_ALL_QUERY_PATH,
        QueryAllPoolReservesRequest::from(req),
    )
}

/// Retrieves a `PoolReserves` by pool reserves key (pair_id + token_in + tick_index + fee).
pub fn get_pool_reserves(
    deps: Deps,
    req: GetPoolReservesRequest,
) -> StdResult<GetPoolReservesResponse> {
    make_stargate_query(
        deps,
        POOL_RESERVES_QUERY_PATH,
        QueryGetPoolReservesRequest::from(req),
    )
}

/// Queries the simulated result of a multihop swap.
pub fn get_estimate_multi_hop_swap(
    deps: Deps,
    req: EstimateMultiHopSwapRequest,
) -> StdResult<EstimateMultiHopSwapResponse> {
    make_stargate_query(
        deps,
        ESTIMATE_MULTI_HOP_SWAP_QUERY_PATH,
        QueryEstimateMultiHopSwapRequest::from(req),
    )
}

/// Queries the simulated result of a limit order placement.
pub fn get_estimate_place_limit_order(
    deps: Deps,
    req: EstimatePlaceLimitOrderRequest,
) -> StdResult<EstimatePlaceLimitOrderResponse> {
    make_stargate_query(
        deps,
        ESTIMATE_PLACE_LIMIT_ORDER_QUERY_PATH,
        QueryEstimatePlaceLimitOrderRequest::from(req),
    )
}

/// Queries a pool by pair, tick and fee.
pub fn get_pool(deps: Deps, req: PoolRequest) -> StdResult<PoolResponse> {
    make_stargate_query(deps, POOL_QUERY_PATH, QueryPoolRequest::from(req))
}

/// Queries a pool by ID.
pub fn get_pool_by_id(deps: Deps, req: PoolByIdRequest) -> StdResult<PoolResponse> {
    make_stargate_query(deps, POOL_BY_ID_QUERY_PATH, QueryPoolByIdRequest::from(req))
}

/// Queries a `PoolMetadata` by ID.
pub fn get_pool_metadata(
    deps: Deps,
    req: GetPoolMetadataRequest,
) -> StdResult<GetPoolMetadataResponse> {
    make_stargate_query(
        deps,
        POOL_METADATA_QUERY_PATH,
        QueryGetPoolMetadataRequest::from(req),
    )
}

/// Queries a list of `PoolMetadata` items.
pub fn get_pool_metadata_all(
    deps: Deps,
    req: AllPoolMetadataRequest,
) -> StdResult<AllPoolMetadataResponse> {
    make_stargate_query(
        deps,
        POOL_METADATA_ALL_QUERY_PATH,
        QueryAllPoolMetadataRequest::from(req),
    )
}
