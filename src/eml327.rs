use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

pub struct Elm327Connection {
    stream: TcpStream,
}

impl Elm327Connection {
    pub async fn connect(address: &str) -> io::Result<Self> {
        let stream = TcpStream::connect(address).await?;
        let mut connection = Elm327Connection { stream };
        
        connection.initialize().await?;
        
        Ok(connection)
    }

    async fn initialize(&mut self) -> io::Result<()> {
        // Send initialization commands. Adjust as needed for your specific setup.
        self.send_command("ATZ\r").await?; // Reset
        self.send_command("ATE0\r").await?; // Echo off

        Ok(())
    }

    async fn send_command(&mut self, command: &str) -> io::Result<()> {
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }

    async fn read_response(&mut self) -> io::Result<String> {
        let mut response = String::new();
        self.stream.read_to_string(&mut response).await?;
        Ok(response)
    }
}