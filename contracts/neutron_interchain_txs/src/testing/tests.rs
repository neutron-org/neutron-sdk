use std::marker::PhantomData;

use crate::{
    contract::query_errors_queue,
    storage::{add_error_to_queue, read_errors_from_queue, ERRORS_QUEUE},
};

use cosmwasm_std::{
    from_binary,
    testing::{MockApi, MockQuerier, MockStorage},
    OwnedDeps,
};

use neutron_sdk::bindings::query::NeutronQuery;

pub fn mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier, NeutronQuery> {
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockQuerier::default(),
        custom_query_type: PhantomData,
    }
}

#[test]
fn test_query_errors_queue() {
    let mut deps = mock_dependencies();

    let result = query_errors_queue(deps.as_ref()).unwrap();
    let result: Vec<(Vec<u8>, String)> = from_binary(&result).unwrap();

    assert_eq!(0, result.len());

    let error_msg = "Error message".to_string();

    ERRORS_QUEUE
        .save(&mut deps.storage, 0u32, &error_msg)
        .unwrap();

    let result = query_errors_queue(deps.as_ref()).unwrap();
    let result: Vec<(Vec<u8>, String)> = from_binary(&result).unwrap();

    assert_eq!(1, result.len());
}

#[test]
fn test_errors_queue() {
    let mut store = MockStorage::new();

    let errors = read_errors_from_queue(&store);
    let errors = errors.unwrap();

    assert_eq!(0, errors.len());

    let error = "some error message".to_string();

    add_error_to_queue(&mut store, error.clone()).unwrap();

    let errors = read_errors_from_queue(&store);
    let errors = errors.unwrap();

    assert_eq!(1, errors.len());
    assert_eq!(errors, vec![(0u32.to_be_bytes().to_vec(), error.clone())]);

    add_error_to_queue(&mut store, error.clone()).unwrap();
    add_error_to_queue(&mut store, error.clone()).unwrap();

    let errors = read_errors_from_queue(&store);
    let errors = errors.unwrap();

    assert_eq!(3, errors.len());
    assert_eq!(
        errors,
        vec![
            (0u32.to_be_bytes().to_vec(), error.clone()),
            (1u32.to_be_bytes().to_vec(), error.clone()),
            (2u32.to_be_bytes().to_vec(), error)
        ]
    );
}
