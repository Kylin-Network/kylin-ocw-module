use sp_core::offchain::testing;

//Constant to be used in test cases
pub const TEST_PHRASE: &str =
    "news slush supreme milk chapter athlete soap sausage put clutch what kitten";
pub const TEST_FEED_NAME: &[u8; 9] = b"test_feed";
pub const TEST_SAMPLE_DATA: &[u8; 13] = b"{sample_data}";
pub const BTC_USD: &[u8; 7] = b"btc_usd";
pub const TEST_RESPONSE: &[u8; 15] = br#"{"USD": 155.23}"#;
pub const ALICE: &str = "Alice";
pub const BOB: &str = "Bob";


pub fn mock_query_response(state: &mut testing::OffchainState) {
    let pending_request = testing::PendingRequest {
        // 99d0a3ca7bf589dc79931a5e310bced3d977906bc2896ea1ceca5a824e91a951
        method: "GET".into(),
        uri: "https://api.kylin-node.co.uk/query?hash=99d0a3ca7bf589dc79931a5e310bced3d977906bc2896ea1ceca5a824e91a951".into(),
        response: Some(br#"{"USD": 155.23}"#.to_vec()),
        sent: true,
        ..Default::default()
    };
    state.expect_request(pending_request);
}

pub fn mock_post_response(state: &mut testing::OffchainState) {

    let sample_body: Vec<u8> = vec![
        123, 10, 32, 32, 32, 32, 34, 100, 97, 116, 97, 34, 58, 32, 123, 10, 32, 32, 32, 32, 32, 32,
        32, 32, 34, 112, 97, 114, 97, 95, 105, 100, 34, 58, 32, 110, 117, 108, 108, 44, 10, 32, 32,
        32, 32, 32, 32, 32, 32, 34, 97, 99, 99, 111, 117, 110, 116, 95, 105, 100, 34, 58, 32, 34,
        100, 52, 51, 53, 57, 51, 99, 55, 49, 53, 102, 100, 100, 51, 49, 99, 54, 49, 49, 52, 49, 97,
        98, 100, 48, 52, 97, 57, 57, 102, 100, 54, 56, 50, 50, 99, 56, 53, 53, 56, 56, 53, 52, 99,
        99, 100, 101, 51, 57, 97, 53, 54, 56, 52, 101, 55, 97, 53, 54, 100, 97, 50, 55, 100, 34,
        44, 10, 32, 32, 32, 32, 32, 32, 32, 32, 34, 114, 101, 113, 117, 101, 115, 116, 101, 100,
        95, 98, 108, 111, 99, 107, 95, 110, 117, 109, 98, 101, 114, 34, 58, 32, 48, 44, 10, 32, 32,
        32, 32, 32, 32, 32, 32, 34, 112, 114, 111, 99, 101, 115, 115, 101, 100, 95, 98, 108, 111,
        99, 107, 95, 110, 117, 109, 98, 101, 114, 34, 58, 32, 48, 44, 10, 32, 32, 32, 32, 32, 32,
        32, 32, 34, 114, 101, 113, 117, 101, 115, 116, 101, 100, 95, 116, 105, 109, 101, 115, 116,
        97, 109, 112, 34, 58, 32, 48, 44, 10, 32, 32, 32, 32, 32, 32, 32, 32, 34, 112, 114, 111,
        99, 101, 115, 115, 101, 100, 95, 116, 105, 109, 101, 115, 116, 97, 109, 112, 34, 58, 32,
        48, 44, 10, 32, 32, 32, 32, 32, 32, 32, 32, 34, 112, 97, 121, 108, 111, 97, 100, 34, 58,
        32, 34, 123, 115, 97, 109, 112, 108, 101, 95, 100, 97, 116, 97, 125, 34, 44, 10, 32, 32,
        32, 32, 32, 32, 32, 32, 34, 102, 101, 101, 100, 95, 110, 97, 109, 101, 34, 58, 32, 34, 112,
        114, 105, 99, 101, 95, 102, 101, 101, 100, 105, 110, 103, 34, 44, 10, 32, 32, 32, 32, 32,
        32, 32, 32, 34, 117, 114, 108, 34, 58, 32, 34, 104, 116, 116, 112, 115, 58, 47, 47, 97,
        112, 105, 46, 107, 121, 108, 105, 110, 45, 110, 111, 100, 101, 46, 99, 111, 46, 117, 107,
        47, 112, 114, 105, 99, 101, 115, 63, 99, 117, 114, 114, 101, 110, 99, 121, 95, 112, 97,
        105, 114, 115, 61, 98, 116, 99, 95, 117, 115, 100, 34, 10, 32, 32, 32, 32, 125, 44, 10, 32,
        32, 32, 32, 34, 104, 97, 115, 104, 34, 58, 32, 34, 57, 57, 100, 48, 97, 51, 99, 97, 55, 98,
        102, 53, 56, 57, 100, 99, 55, 57, 57, 51, 49, 97, 53, 101, 51, 49, 48, 98, 99, 101, 100,
        51, 100, 57, 55, 55, 57, 48, 54, 98, 99, 50, 56, 57, 54, 101, 97, 49, 99, 101, 99, 97, 53,
        97, 56, 50, 52, 101, 57, 49, 97, 57, 53, 49, 34, 10, 125,
    ];

    let mut pending_request = testing::PendingRequest {
        method: "POST".into(),
        uri: "https://api.kylin-node.co.uk/submit".into(),
        body: sample_body,
        response: Some(br#"{"USD": 155.23}"#.to_vec()),
        sent: true,
        ..Default::default()
    };

    pending_request
        .headers
        .push(("x-api-key".into(), "test_api_key".into()));
    pending_request
        .headers
        .push(("content-type".into(), "application/json".into()));
    state.expect_request(pending_request);
}

pub fn mock_submit_response(state: &mut testing::OffchainState) {
    state.expect_request(testing::PendingRequest {
        method: "GET".into(),
        uri: "https://api.kylin-node.co.uk/prices?currency_pairs=btc_usd".into(),
        response: Some(TEST_RESPONSE.to_vec()),
        sent: true,
        ..Default::default()
    });
}
