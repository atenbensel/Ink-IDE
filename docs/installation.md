# Installing the Ink! IDE Integration

## Prerequisites

Before installing the Ink! IDE integration, ensure you have the following prerequisites installed:

Aleph Zero SDK
Ink! runtime and development tools

## Installation Steps

Clone the IDE Integration Repository: Clone this repository to your local machine.

```Bash
git clone https://github.com/paritytech/ink/ide
```

Install IDE Integration: Navigate to the ide directory and run the following command:

```Bash
cargo build --release
```

Launch IDE Integration: Execute the following command to launch the IDE integration:

```Bash
./target/release/ink_ide
```
Verifying Installation

Once the IDE integration is launched, you can verify its installation by checking the version information.

```Bash
./target/release/ink_ide --version
```

This should display the installed version of the IDE integration.