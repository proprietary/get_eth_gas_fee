use serde::{Deserialize, Serialize};
use serde_json::{json};

// #[derive(Debug, Serialize, Deserialize)]
// struct GetBlockByNumberResponse {
// 	jsonrpc: String,
// 	id: String,
// 	#[serde(rename = "transactionsRoot")]
// 	transactions_root: String,
// 	result: GetBlockByNumberResponseResult,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct GetBlockByNumberResponseResult {
// 	#[serde(rename = "baseFeePerGas")]
// 	base_fee_per_gas: String,
// 	difficulty: String,
// 	#[serde(rename = "extraData")]
// 	extra_data: String,
// 	#[serde(rename = "gasLimit")]
// 	gas_limit: String,
// 	#[serde(rename = "gasUsed")]
// 	gas_used: String,
// }

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
	jsonrpc: String,
	id: String,
	method: String,
	params: Vec<serde_json::Value>,
}

const ETH_RPC_URL: &'static str = "https://cloudflare-eth.com";

pub fn base_fee_per_gas(block_number: u64) -> u64 {
	let block_number_as_str = format!("0x{:x}", block_number);
	let data = JsonRpcRequest{
		jsonrpc: "2.0".into(),
		id: "dontcare".into(),
		method: "eth_getBlockByNumber".into(),
		params: vec![json!(block_number_as_str), json!(true)],
	};
	let client = reqwest::blocking::Client::new();
	let body: serde_json::Value = client.post(ETH_RPC_URL)
		.body(serde_json::to_string(&data).unwrap())
		.send().unwrap()
		.json().unwrap();
	let body = body.as_object().unwrap();
	let result = body["result"].as_object().unwrap();
	let base_fee_per_gas_str = result["baseFeePerGas"].as_str().unwrap();
	let base_fee_per_gas_str = base_fee_per_gas_str.trim_start_matches("0x");
	let base_fee_u64 = u64::from_str_radix(base_fee_per_gas_str, 16).unwrap();
	return base_fee_u64 / 1_000_000_000;
}

pub fn block_number() -> u64 {
	let data = JsonRpcRequest{
		jsonrpc: "2.0".into(),
		id: "dontcare".into(),
		method: "eth_blockNumber".into(),
		params: vec![],
	};
	let client = reqwest::blocking::Client::new();
	let body: serde_json::Value = client.post(ETH_RPC_URL)
		.body(serde_json::to_string(&data).unwrap())
		.send()
		.unwrap()
		.json()
		.unwrap();
	let body = body.as_object().unwrap();
	let result = body["result"].as_str().unwrap();
	let block_number = parse_hex_str(result).unwrap();
	block_number
}

fn parse_hex_str(s: &str) -> Result<u64, std::num::ParseIntError> {
	let s = s.trim_start_matches("0x");
	return u64::from_str_radix(s, 16);
}
