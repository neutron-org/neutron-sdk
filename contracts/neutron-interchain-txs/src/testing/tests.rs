// use cosmwasm_std::{testing::mock_env, Binary};
// use interchain_txs::{msg::SudoMsg, storage::RequestPacket};

// use crate::contract::sudo;

// use super::mock_querier::mock_dependencies as dependencies;

// #[test]
// fn test_sudo_response() {
//     let mut deps = dependencies(&[]);
//     let msg = SudoMsg::Response { request: RequestPacket{
//         sequence: Some(2),
//         source_port: Some("icacontroller-cosmos14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s4hmalr".to_string()),
//         source_channel: Some("channel-0".to_string()),
//         destination_port: Some("icahost".to_string()),
//         destination_channel: Some("channel-0".to_string()),
//         data: Some("eyJkYXRhIjoiQ3JFQkNpVXZZMjl6Ylc5ekxuTjBZV3RwYm1jdWRqRmlaWFJoTVM1TmMyZFZibVJsYkdWbllYUmxFb2NCQ2tGamIzTnRiM014ZERWeVpXcHFNR3BuZHpZelptTXliV3B4ZUdSeGFtZzJZMlJvYUhkaGF6Wnljbk40TlhOcWRXTTJibkJyY0hGa05UVXpjelV3WjNGd2JCSTBZMjl6Ylc5emRtRnNiM0JsY2pGeGJtc3lialJ1Ykd0d2R6bDRabkZ1ZEd4aFpHZzNOSGMyZFdwMGRXeDNibTE0Ym1nemF4b01DZ1Z6ZEdGclpSSUROVEF3IiwibWVtbyI6IiIsInR5cGUiOiJUWVBFX0VYRUNVVEVfVFgifQ==".to_string()),
//         timeout_height: None,
//         timeout_timestamp: Some(1658682439537778000u64)
//     }, data: "CjYKJS9jb3Ntb3Muc3Rha2luZy52MWJldGExLk1zZ1VuZGVsZWdhdGUSDQoLCMniv5cGEOjt2io=".to_string() };
//     let res = sudo(deps.as_mut(), mock_env(), msg).unwrap();
//     let str_res = std::str::from_utf8(
//         &Binary::from_base64(res.data.unwrap().to_string().as_str())
//             .unwrap()
//             .to_vec(),
//     )
//     .unwrap()
//     .to_string();

//     assert_eq!(str_res, "/cosmos.staking.v1beta1.MsgUndelegate 1659892041")
// }
