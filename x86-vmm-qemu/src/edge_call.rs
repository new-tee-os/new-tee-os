use async_std::{io::BufReader, os::unix::net::UnixListener};

pub struct EdgeCallServer {
    sock: UnixListener,
}

impl EdgeCallServer {
    pub async fn new() -> async_std::io::Result<EdgeCallServer> {
        async_std::fs::remove_file("edge.sock").await?;
        let sock = UnixListener::bind("edge.sock").await?;
        Ok(EdgeCallServer { sock })
    }

    pub async fn listen(&self) -> async_std::io::Result<()> {
        use async_std::stream::StreamExt;
        use futures_lite::io::AsyncBufReadExt;

        log::info!("Listening for edge calls at edge.sock");
        let mut incoming = self.sock.incoming();
        if let Some(stream) = incoming.next().await {
            let stream = stream?;
            let mut reader = BufReader::new(stream);
            loop {
                let mut line = String::new();
                reader.read_line(&mut line).await?;
                if line.is_empty() {
                    break;
                }
                log::info!("Guest says: {}", line);
            }
        }
        Ok(())
    }
}
