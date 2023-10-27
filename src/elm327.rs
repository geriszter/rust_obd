use tokio::net::TcpStream;
use tokio::io::{self, AsyncWriteExt, AsyncReadExt};

pub struct Elm327Connection {
    stream: TcpStream,
}

impl Elm327Connection {

    async fn initialize(&mut self) -> io::Result<String> {
        self.send_command("ATZ\r").await?; // Reset
        let version = self.read_response().await?; 
        self.send_command("ATE0\r").await?; // Echo off

        Ok(version)
    }

    pub async fn connect(address: &str) -> io::Result<(Self, String)> {
        let stream = TcpStream::connect(address).await?;
        let mut connection = Elm327Connection { stream };
        
        let version = connection.initialize().await?;
        
        Ok((connection, version))
    }


    async fn send_command(&mut self, command: &str) -> io::Result<()> {
        self.stream.write_all(command.as_bytes()).await?;
        Ok(())
    }

    async fn read_response(&mut self) -> io::Result<String> {
        let mut buffer = Vec::new();
        let mut byte = [0u8; 1];
    
        loop {
            self.stream.read_exact(&mut byte).await?;
            if byte[0] == b'>' { // end of line
                break;
            }
            buffer.push(byte[0]);
        }
    
        String::from_utf8(buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
    

    //Just for testing
    pub async fn read_coolant_temperature(&mut self) -> i32 {
        if let Err(e) = self.send_command("0105\r").await {
            println!("Failed to send coolant temperature command: {}", e);
            return 0;
        }
        
        let response = match self.read_response().await {
            Ok(res) => res,
            Err(e) => {
                println!("Failed to read coolant temperature response: {}", e);
                return 0;
            }
        };
        
        if let Some(captured) = response.split_whitespace().nth(2) {
            if let Ok(a) = i32::from_str_radix(captured, 16) {
                return a - 40;  // Convert to Celsius
            } else {
                println!("Failed to parse coolant temperature response");
            }
        } else {
            println!("Failed to interpret coolant temperature data");
        }

        return 0;
    }
    
    //Just for testing, alos it should return the array
    pub async fn read_stored_dtc(&mut self) -> Result<(), String> {
        self.send_command("03\r").await.map_err(|_| "Failed to send DTC command".to_string())?;
        let response = self.read_response().await.map_err(|_| "Failed to read DTC response".to_string())?;

        let mut codes: Vec<String> = Vec::new();
        let mut parts = response.split_whitespace().skip(1); // Skip the initial "43" or equivalent

        while let Some(a) = parts.next() {
            if let Some(b) = parts.next() {
                let dtc = format!("P{}{}", a, b);
                codes.push(dtc);
            }
        }

        if codes.is_empty() {
            Err("No DTCs found or failed to interpret DTC data".to_string())
        } else {
            println!("Stored Diagnostic Trouble Codes:");
            for code in &codes {
                println!("{}", code);
            }
            Ok(())
        }
    }
    


}
