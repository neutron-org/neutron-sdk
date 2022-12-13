use obi::{OBIDecode, OBIEncode, OBISchema};

// encode into obi format: https://docs.bandchain.org/technical-specifications/obi.html
pub fn calldata() -> Vec<u8> {
    // const obi = new Obi('{symbols:[string],multiplier:u64}/{rates:[u64]}')
    // const calldata = obi.encodeInput({ symbols: ['ETH'], multiplier: 100 })
    // https://docs.bandchain.org/client-library/pyband/obi.html
    // https://pypi.org/project/pyband/
    let data = Input {
        symbols: vec!["BTC".to_string()],
        multiplier: 1000000,
    };
    data.try_to_vec()
        .ok()
        .expect("calldata should be encoded correctly")
}

#[derive(OBIEncode, OBISchema)]
pub struct Input {
    pub symbols: Vec<String>,
    pub multiplier: u64,
}

#[derive(OBIDecode, OBISchema)]
pub struct Output {
    pub rates: Vec<u64>,
}
