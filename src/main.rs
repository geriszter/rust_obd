use tokio::time::{timeout, Duration};
mod elm327;
mod command;

#[tokio::main]
async fn main() {
    let address = "127.0.0.1:35000"; 

    match timeout(Duration::from_secs(5), elm327::Elm327Connection::connect(address)).await {
        Ok(Ok(mut connection)) => {
            println!("Successfully connected to ELM327 device!");
            println!("ELM327 Version: {}", connection.get_version());

            let cmd = command::OBDCommand::get_command("ENGINE_COOOLANT_TEMPERATURE");

            command::OBDCommand::get_command_info("MAF");

            if let Some(command) = cmd {
                match timeout(Duration::from_secs(10), connection.send_obd_command(&command)).await {
                    Ok(Ok(result)) => {
                        println!("Command Name: {}, Result: {}", command.name, result);
                    }
                    Ok(Err(e)) => {
                        println!("Error sending OBD command: {}", e);
                    },
                    Err(_) => {
                        println!("Timed out while sending OBD command");
                    }
                }
            }

            // // Read Coolant Temperature WITH HEADERS
            // let temp = connection.read_coolant_temperature().await;
            // println!("Coolant Temperature: {}°C", temp);

            // match timeout(Duration::from_secs(5), connection.read_stored_dtc()).await {
            //     Ok(Ok(_)) => println!("Done"),
            //     Ok(Err(e)) => println!("Error reading DTCs: {}", e),
            //     Err(_) => println!("Timed out while reading DTCs"),
            // }
        },
        Ok(Err(e)) => {
            println!("Failed to connect: {}", e);
        },
        Err(_) => {
            println!("Connection Timed Out");
        }
    }
    

    // //command test
    //let cmd = command::OBDCommand::get_command("ENGINE_COOOLANT_TEMPERATURE");

    // if let Some(command) = cmd {
    //     println!("Command Name: {}, cmd: {}", command.name, String::from_utf8_lossy(&command.cmd[..]).to_string());
    // }
    

}
