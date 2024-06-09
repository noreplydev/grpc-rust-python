# Interfacing Rust and Python with gRPC
This repo is a simple example of how to interface Rust and Python using gRPC. The Rust server is a simple service that returns an entity based on a request entity type from the client. The Python client sends a request with an entity type to the server and receives an Entity.

## Running the project
gRPC requires the [protobuf compiler](https://protobuf.dev/downloads) to generate the code for each language. 

You can run `genprotos.sh` to generate the code for Python. In rust, the code is generated at compile time using the `/services/rust/build.rs` file.

Once the code is generated, you can run the grpc rust server with `cargo run --manifest-path services/rust/Cargo.toml` which will be listening on `localhost:50051`. You can then install the python dependencies and run the Python client with `python3 services/python/client.py` which will send a request to the server and print the response (I highly recommend using virtual environments for Python).

To summarize this are the steps: 

1. Generate the code for Python and install the dependencies
```bash
/bin/bash genprotos.sh
pip3 install -r services/python/requirements.txt
```

2. Run the Rust server and the Python client
```bash
cargo run --manifest-path services/rust/Cargo.toml
```

```bash
python3 services/python/client.py
```
