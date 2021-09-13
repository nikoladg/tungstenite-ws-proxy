use futures_util::StreamExt;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() -> Result<(), tungstenite::error::Error> {
    // set up a ws server
    let addr = "127.0.0.1:8080".to_string();
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");

    // handle incoming ws connections
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream) -> Result<(), tungstenite::error::Error> {
    // accept the incoming connection
    let ws_client = tokio_tungstenite::accept_async(stream).await?;
    let (write_client, read_client) = ws_client.split();

    // create an outgoing connection
    let (ws_destination, _) = connect_async("ws://127.0.0.1:8081").await?;
    let (write_destination, read_destination) = ws_destination.split();

    // forward the incoming and outgoing connections
    tokio::spawn(read_client.forward(write_destination));
    tokio::spawn(read_destination.forward(write_client));

    Ok(())
}
