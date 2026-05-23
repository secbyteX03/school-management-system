# School Management Contract

This is a school management system I built on Stellar using Soroban. It helps track students, their classes, and payment history.

## What It Does

The contract lets you do the following:
- Register new students with their name and class
- Make payments like transfers tokens from student to school
- Update a student's class
- Get a student's payment history
- Remove a student from the system

## Building the Contract

Make sure you have the Stellar CLI installed, then run the commmand:

```bash
stellar contract build
```

This will create a WASM file that is ready for deployment

## Running Tests

To test everything run the following commmands:

```bash
cd contracts/school-management
cargo test
```

The tests cover registering students, making payments, updating classes, getting payment history, and removing students.

## Deploying to Testnet

To deploy to the Stellar testnet, you need to configure your Stellar CLI with your identity first using the commands:

```bash
stellar keys generate <identity_name> --network testnet
stellar keys use <identity_name>
stellar keys fund <identity_name> --network testnet
```

First, deploy a token contract for payments you can use the SEP-41 token contract to do this:

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/sep41_token.wasm --network testnet
```

Then deploy the school management contract with the token address:

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/school_management.wasm --network testnet -- --admin <your_address> --token <token_contract_address>
```

**Deployed Contract**: `CDYOJNCVLNZUHBAEHYHLLQR4JQMFKX7CQX7M5774YF5HC7HZHXJVLARN`
**Token Contract Used**: `CBT3JN55CGPJRWQQCQRJWRZ2EJB5K3AHWAJYCOTSXUOS67UTYLTPEHQ3`

## Project Structure

```
contracts/school-management/
├── src/
│   ├── lib.rs                  # Module setup
│   ├── school_management.rs    # Main contract code
│   ├── storage.rs              # Data structures and storage keys
│   ├── events.rs               # Event structures
│   ├── error.rs                # Error types
│   └── test.rs                 # Tests
└── Cargo.toml
```
