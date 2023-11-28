# Ink! IDE Integration for Aleph Zero

## Introduction

This project provides an IDE integration for the Ink! programming language, making it easier to develop and deploy smart contracts on the Aleph Zero blockchain. The integration seamlessly integrates with your preferred IDE, providing real-time feedback, code completion, syntax highlighting, debugging capabilities, and contract deployment and management tools.

## Features

* **Language Server Protocol (LSP):** Real-time feedback, code completion, syntax highlighting, and other development aids.

* **Debugging and Visualization:** Step through Ink! code, inspect variables, and visualize data flow.

* **Contract Deployment and Management:** Compile, generate transactions for, and monitor the status of Ink! contracts.

* **Smart Contract Libraries and Templates:** Access a collection of pre-built smart contract libraries and templates to accelerate development and encourage code reuse.

## Installation

1. **Install Aleph Zero:** Follow the instructions on the Aleph Zero website to install the Aleph Zero SDK and set up your development environment.

2. **Install Ink!:** Follow the instructions on the Ink! website to install the Ink! runtime and development tools.

3. **Clone the IDE Integration Repository:** Clone this repository to your local machine.

4. **Install IDE Integration:** Navigate to the `ide` directory and run the following command:

```bash
cargo build --release
```
5. Launch IDE Integration: Execute the following command to launch the IDE integration:

   ./target/release/ink_ide

## Usage

1. **Connect to Aleph Zero:** Open the IDE integration and connect to your Aleph Zero node.

2. **Create a New Project:** Select the "New Project" option and choose the Ink! language.

3. **Write Ink! Contracts:** Write Ink! contracts using the IDE's editor with real-time feedback, code completion, and syntax highlighting.

4. **Debug Contracts:** Set breakpoints, step through code, and inspect variables to debug your Ink! contracts.

5. **Deploy Contracts:** Compile and deploy Ink! contracts to the Aleph Zero blockchain.

6. **Manage Contracts:** Monitor the status of deployed Ink! contracts and interact with them using the IDE's tools.

## Contributing

We welcome contributions to the Ink! IDE Integration for Aleph Zero. Please refer to the [contributing guidelines](CONTRIBUTING.md) for more information on how to contribute.

## License

This project is licensed under the MIT License. Please refer to the [LICENSE](LICENSE) file for more information.


