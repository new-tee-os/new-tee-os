use async_std::{
    io::prelude::{ReadExt, WriteExt},
    os::unix::net::{UnixListener, UnixStream},
};
use hal::edge::{AsyncEdgeStream, EdgeMemory};

pub struct EdgeCallServer {
    sock: UnixListener,
}

struct EdgeCallClient(UnixStream);

impl EdgeCallServer {
    pub async fn new() -> async_std::io::Result<EdgeCallServer> {
        async_std::fs::remove_file("edge.sock").await?;
        let sock = UnixListener::bind("edge.sock").await?;
        Ok(EdgeCallServer { sock })
    }

    pub async fn listen(&self) -> async_std::io::Result<()> {
        use async_std::stream::StreamExt;

        log::info!("Listening for edge calls at edge.sock");
        let mut incoming = self.sock.incoming();
        if let Some(stream) = incoming.next().await {
            let mut edge_stream = EdgeCallClient(stream?);
            let mut edge_mem = EdgeMemory::new();
            loop {
                edge_mem.deserialize_async(&mut edge_stream).await?;
                unsafe {
                    edge_responder::handle_edge_call(&mut edge_mem);
                }
                edge_mem.serialize_async(&mut edge_stream).await?;
            }
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncEdgeStream<std::io::Error> for EdgeCallClient {
    async fn read_bulk_async(&mut self, buf: &mut [u8]) -> Result<(), std::io::Error> {
        self.0.read_exact(buf).await
    }

    async fn write_bulk_async(&mut self, buf: &[u8]) -> Result<(), std::io::Error> {
        self.0.write_all(buf).await
    }
}
