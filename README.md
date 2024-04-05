## Description

This is a simple Test Case of Nexus zkVM code. This project will run two main Nexus zkVM functions, `prove` and `verify`. It includes 3 test cases, `Hello World`, `Addition`, and `Fibonacci Sequence`, that will be executed using Nexus zkVM cargo.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Nexus zkVM](https://docs.nexus.xyz/zkVM/quick-start)

## Directory Structure

```text
project_name
├── Cargo.lock
├── Cargo.toml                  <-- [Has nexus cargo dependencies]
├── LICENSE
├── README.md
├── src
│   ├── addition.rs
│   ├── fibonacci.rs
│   ├── hello_world.rs
│   └── main.rs                 <-- [The main test code that imports 3 test cases]
└── target
    ...
```

## How To Run

1. Before running the test, in the `/src/main.rs`, you can comment and uncomment your desired test case. The default will run Hello World.

2. Run `prove generation` 
```bash
cargo nexus prove
```
If run correctly, the console prints the corresponding test message. When finished, it will create a binary file named `nexus-proof` in the project root.

3. Run `verify proof`
```bash
cargo nexus verify
```
By default, verify will look for a `nexus-proof` file in the root project.

## Run With Configuration

- Using `k number`. The k number denotes how many NexusVM instructions are batched into each prover step. The default value is 16. **If the prove uses k=8, the verify should use k=8 too**.  
  - Run `prove generation` with k number
    ```bash
    cargo nexus prove -k=8
    ```

  - Run `verify proof` with k number
    ```bash
    cargo nexus verify -k=8
    ```

- Run `verify proof` with path
```bash
cargo nexus verify /your/nexus/proof/path
```

## References

- [Nexus zkVM](https://nexus.xyz/)
- [Nexus zkVM Documentation](https://docs.nexus.xyz/)
- [Nexus zkVM Github](https://github.com/nexus-xyz/nexus-zkvm)