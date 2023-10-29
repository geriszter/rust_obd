mod elm327;
mod command;

#[tokio::main]
async fn main() {
    let address = "192.168.0.10:35000"; 

    match elm327::Elm327Connection::connect(address).await {
        Ok((mut connection, version)) => {
            println!("Successfully connected to ELM327 device!");
            println!("ELM327 Version: {}", version.trim());

            // Read Coolant Temperature
            let temp = connection.read_coolant_temperature().await;
            println!("Coolant Temperature: {}°C", temp);


            match connection.read_stored_dtc().await {
                Ok(_) => println!("Done"),
                Err(e) => println!("Error reading DTCs: {}", e),
            }


        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    

    // //command test
    let cmd = command::OBDCommand::get_command("ENGINE_COOOLANT_TEMPERATURE");

    if let Some(command) = cmd {
        println!("Command Name: {}, cmd: {}", command.name, String::from_utf8_lossy(&command.cmd[..]).to_string());
    }
    

}
