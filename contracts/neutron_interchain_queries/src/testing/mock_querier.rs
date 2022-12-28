use std::collections::HashMap;
use std::marker::PhantomData;

use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{
    from_slice, Binary, Coin, ContractResult, CustomQuery, FullDelegation, OwnedDeps, Querier,
    QuerierResult, QueryRequest, SystemError, SystemResult, Uint128, Validator,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use neutron_sdk::bindings::query::NeutronQuery;

pub const MOCK_CONTRACT_ADDR: &str = "cosmos2contract";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CustomQueryWrapper {}

// implement custom query
impl CustomQuery for CustomQueryWrapper {}

pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier, NeutronQuery> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(contract_addr, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: PhantomData,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<NeutronQuery>,
    query_reponses: HashMap<u64, Binary>,
    registred_queries: HashMap<u64, Binary>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<NeutronQuery> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return QuerierResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                });
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<NeutronQuery>) -> QuerierResult {
        match &request {
            QueryRequest::Custom(NeutronQuery::InterchainQueryResult { query_id }) => {
                SystemResult::Ok(ContractResult::Ok(
                    (*self.query_reponses.get(query_id).unwrap()).clone(),
                ))
            }
            QueryRequest::Custom(NeutronQuery::RegisteredInterchainQuery { query_id }) => {
                SystemResult::Ok(ContractResult::Ok(
                    (*self.registred_queries.get(query_id).unwrap()).clone(),
                ))
            }
            QueryRequest::Custom(NeutronQuery::RegisteredInterchainQueries {
                owners: _owners,
                connection_id: _connection_id,
                pagination: _pagination,
            }) => {
                todo!()
            }
            QueryRequest::Custom(NeutronQuery::InterchainAccountAddress { .. }) => {
                todo!()
            }
            _ => self.base.handle_query(request),
        }
    }

    pub fn _update_staking(
        &mut self,
        denom: &str,
        validators: &[Validator],
        delegations: &[FullDelegation],
    ) {
        self.base.update_staking(denom, validators, delegations);
    }

    pub fn add_query_response(&mut self, query_id: u64, response: Binary) {
        self.query_reponses.insert(query_id, response);
    }
    pub fn add_registred_queries(&mut self, query_id: u64, response: Binary) {
        self.registred_queries.insert(query_id, response);
    }
}

#[derive(Clone, Default)]
pub struct BalanceQuerier {
    _balances: HashMap<String, Coin>,
}

#[derive(Clone, Default)]
pub struct TokenQuerier {
    _balances: HashMap<String, HashMap<String, Uint128>>,
}

impl WasmMockQuerier {
    pub fn new(base: MockQuerier<NeutronQuery>) -> Self {
        WasmMockQuerier {
            base,
            query_reponses: HashMap::new(),
            registred_queries: HashMap::new(),
        }
    }
}
