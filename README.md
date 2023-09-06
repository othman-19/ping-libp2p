Ping pong between two nodes created with Rust and Libp2p.

To run the example, follow these steps:

1. In a first terminal window, run the following command:
`cargo run`
This command starts a node and prints the PeerId and the listening addresses, such as Listening on "/ip4/0.0.0.0/tcp/24915".

2. In a second terminal window, start a new instance of the example with the following command:
`cargo run -- /ip4/127.0.0.1/tcp/24915`.
Replace /ip4/127.0.0.1/tcp/24915 with the listen address of the first node obtained from the first terminal window.

3. The two nodes will establish a connection, negotiate the ping protocol, and begin pinging each other.
