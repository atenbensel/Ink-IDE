use ink::prelude::*;
use aleph-zero-sdk::AlephZeroSdk;

struct DeploymentManager {
    sdk: AlephZeroSdk,
}

impl DeploymentManager {
    fn new() -> Self {
        DeploymentManager {
            sdk: AlephZeroSdk::new().unwrap(),
        }
    }

    fn deploy_contract(&self, contract_path: &str) -> Result<String, ()> {
        // Read the contract file
        let contract_code = std::fs::read_to_string(contract_path).unwrap();

        // Compile the contract
        let compiled_contract = self.sdk.compile_contract(&contract_code).unwrap();

        // Deploy the contract
        let contract_address = self.sdk.deploy_contract(&compiled_contract).unwrap();

        Ok(contract_address)
    }

    fn get_contract_status(&self, contract_address: &str) -> Result<String, ()> {
        // Query the contract status
        let contract_status = self.sdk.get_contract_status(contract_address).unwrap();

        Ok(format!("{:?}", contract_status))
    }

    fn send_transaction(&self, contract_address: &str, method_name: &str, args: ink::Value) -> Result<(), ()> {
        // Prepare the transaction
        let transaction = self.sdk.create_transaction(contract_address, method_name, args).unwrap();

        // Send the transaction
        self.sdk.send_transaction(&transaction).unwrap();

        Ok(())
    }
}

fn main() {
    let manager = DeploymentManager::new();

    // Deploy the contract
    let contract_address = manager.deploy_contract("src/main.ink").unwrap();
    println!("Contract deployed to: {}", contract_address);

    // Get contract status
    let contract_status = manager.get_contract_status(&contract_address).unwrap();
    println!("Contract status: {}", contract_status);

    // Send a transaction
    manager.send_transaction(&contract_address, "increment", ink::Value::Int(1)).unwrap();
    println!("Transaction sent successfully");
}