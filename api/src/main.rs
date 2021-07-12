use futures_util::{stream::SplitSink, SinkExt, StreamExt};

type GenericError = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), GenericError> {
    const WS_SERVER_ADDR : &str = "127.0.0.1:8081";
    let listener = tokio::net::TcpListener::bind(WS_SERVER_ADDR).await?;
    println!("Listening on: {}", WS_SERVER_ADDR);
    let (channel_sender, _channel_reader) = tokio::sync::broadcast::channel::<String>(5000);
    while let Ok((tcp_stream, addr)) = listener.accept().await {
        tokio::spawn(connection_handler(tcp_stream, channel_sender.clone(), addr));
    }
    Ok(())
}

async fn connection_handler(
    stream: tokio::net::TcpStream,
    channel_sender : tokio::sync::broadcast::Sender<String>,
    addr: std::net::SocketAddr
) -> tungstenite::Result<()> {
    //type of event sources
    let mut char_room_reader = channel_sender.subscribe();
    let (mut ws_sender, mut ws_receiver) = tokio_tungstenite::accept_async(stream).await?.split();
    let mut interval = tokio::time::interval(std::time::Duration::from_millis(500));
    let mut timestamp = std::time::Instant::now();

    println!("client {:?} connected(ws)", addr);
    //process mutual exclusive events for the client
    use tungstenite::Message::Text;
    use tungstenite::Message::Binary;
    use tungstenite::Message::Ping;
    use tungstenite::Message::Pong;
    use tungstenite::Message::Close;
    loop {
      tokio::select! {
          // the client send msg to server
          Some(msg) = ws_receiver.next() => {
              if let Ok(msg) = msg {
                match msg {
                    Text(msg) => {
                        let msg = format!("{:?} : {}", addr, msg);
                        println!("{}",msg);
                        channel_sender.send(msg).unwrap();
                    },
                    Ping(data) => {
                        ws_sender.send(Pong(data)).await.unwrap();
                    },
                    Pong(data) => {
                        // update the heartbeat counter for the client
                        // we can use this estimate client's round trip latency
                        // println!("client {:?} : PONG {:?}",addr, data);
                        timestamp = std::time::Instant::now();
                    }
                    _ => break,
                }
              } else  {
                  println!("{:?}", msg);
                  break;
              }
          },
          // the client should get message from the other, then broadcast
          Ok(msg) = char_room_reader.recv() => {
              ws_sender.send(Text(msg)).await.unwrap();
          },
          // client should get hearbeat for configured units of time
          _ = interval.tick() => {
              let elapsed = timestamp.elapsed().as_secs();
              println!("timestamp {:?} : {:?}",addr, elapsed);
              if elapsed > 8 {
                  println!("client {:?} : DEADCONN",addr);
                  break;
              }
              ws_sender.send(Ping("?".into())).await.unwrap();
          },
      }
    }
    println!("client {:?} terminated(ws)", addr);
    Ok(())
}
