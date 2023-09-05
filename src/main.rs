use libp2p::{identity, PeerId, ping, development_transport};
use libp2p::swarm::{keep_alive, NetworkBehaviour};
use std::error::Error;

/// Our network behaviour.
///
/// For illustrative purposes, this includes the [`KeepAlive`](behaviour::KeepAlive) behaviour
/// so a continuous sequence of pings can be observed.
#[derive(NetworkBehaviour, Default)]
struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    let transport = development_transport(local_key).await?;

    let behaviour = Behaviour::default();

    Ok(())
}