use get_eth_gas_fee::*;
use std::io::Write;

fn main() {
    let latest_block = block_number();
    let base_fee = base_fee_per_gas(latest_block);
    print!("{}", base_fee);
    std::io::stdout().flush().unwrap();
}
