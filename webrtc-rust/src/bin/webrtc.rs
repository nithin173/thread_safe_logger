use std::sync::Arc;
use webrtc::{
    api::APIBuilder,
    peer_connection::configuration::RTCConfiguration,
    peer_connection::RTCPeerConnection,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let api = APIBuilder::new().build();
    let config = RTCConfiguration::default();
    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    let pc_clone = Arc::clone(&peer_connection); // Clone before closure

    peer_connection.on_ice_candidate(Box::new(move |candidate| {
        let pc2 = Arc::clone(&pc_clone); // Clone inside closure
        Box::pin(async move {
            if let Some(c) = candidate {
                if let Ok(json) = c.to_json() {
                    println!("New ICE candidate: {:?}", json);
                }
            }
        })
    }));

    println!("WebRTC PeerConnection created!");
    Ok(())
}
