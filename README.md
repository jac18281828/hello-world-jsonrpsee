# Hello World JSON-RPC Example with `jsonrpsee`

This is a simple example of a JSON-RPC server and client implemented in Rust using the `jsonrpsee` library. The server exposes a single method `say_hello` that returns a greeting message, and the client can call this method to receive the greeting.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) installed on your system.
- `cargo`, the Rust package manager, is available.

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/jac18281828/hello-world-jsonrpsee.git
cd hello-world-jsonrpsee
```

### VSCode

`Reopen In Container`

### 2. Build the Project

Use Cargo to build the project:

```bash
cargo build
```

### 3. Run the Server

Start the JSON-RPC server:

```bash
cargo run --bin rpcserver
```

The server will start and listen for JSON-RPC requests.

## Example: Calling the JSON-RPC Method with `curl`

You can also interact with the JSON-RPC server using `curl`. Here's an example of how to call the `say_hello` method:

```bash
curl -X POST http://127.0.0.1:XXXXX \
-H "Content-Type: application/json" \
-d '{
    "jsonrpc": "2.0",
    "method": "say_hello",
    "params": [],
    "id": 1
}'
```

The expected response is:

```json
{
    "jsonrpc": "2.0",
    "result": "Hello there!!",
    "id": 1
}
```

## Project Structure

- **`rpcserver/src`**: The JSON-RPC server implementation.
- **`core_rpc/src`**: The server library 
- **`Cargo.toml`**: The project configuration file, including dependencies.

## Dependencies

This project uses the following main dependencies:

- [`jsonrpsee`](https://crates.io/crates/jsonrpsee) - A JSON-RPC library for Rust.
- [`tokio`](https://crates.io/crates/tokio) - An asynchronous runtime for Rust.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
