pub struct OBDCommand {
    pub name: &'static str, //change back to priv later
    description: &'static str,
    pub cmd: &'static [u8; 4], //change back to priv later
    bytes: u8,
    decoder: DecoderFunction,
}

enum DecoderFunction {
    Default,
    PID,
    Status,
    SingleDTC,
    FuelStatus,
    Percent,
    Temperature,
    PercentCentered,
    FuelPressure,
    Pressure,
    UAS(u8),
    TimingAdvance,
    AirStatus,
    O2Sensors,
    SensorVoltage,
    OBdCompliance,
    O2SensorsAlt,
    AuxInputStatus,
    SensorVoltageBig,
    CurrentCentered,
    EvapPressure,
    Load,
    FuelSystem,
    // ...
}

//there are 78 commands
static MODE1: [OBDCommand; 7] = [
    OBDCommand { name: "PIDS_A", description: "Supported PIDs [01-20]", cmd: b"0100", bytes: 6, decoder: DecoderFunction::PID },
    OBDCommand { name: "STATUS", description: "Status since DTCs cleared", cmd: b"0101", bytes: 6, decoder: DecoderFunction::Status },
    OBDCommand { name: "FREEZE_DTC", description: "Freeze DTC", cmd: b"0102", bytes: 2, decoder: DecoderFunction::Default },
    OBDCommand { name: "FUEL_SYSTEM", description: "Fuel system status", cmd: b"0103", bytes: 2, decoder: DecoderFunction::FuelSystem },
    OBDCommand { name: "LOAD", description: "Calculated engine load", cmd: b"0104", bytes: 1, decoder: DecoderFunction::Load },
    OBDCommand { name: "ENGINE_COOOLANT_TEMPERATURE", description: "Engine coolant temperature", cmd: b"0105", bytes: 1, decoder: DecoderFunction::Temperature },
    OBDCommand { name: "PLACEHOLDER", description: "Placeholder", cmd: b"01XX", bytes: 1, decoder: DecoderFunction::Default },
];

impl OBDCommand {
    
    
    pub fn decode_data(command: &OBDCommand, raw_data: &[u8]) -> String {
        match command.decoder {
            DecoderFunction::PID => todo!(),
            DecoderFunction::Status => Self::decode_status(raw_data),
            DecoderFunction::Temperature => Self::decode_temperature(raw_data).to_string(),
            DecoderFunction::Default => todo!(),
            DecoderFunction::SingleDTC => todo!(),
            DecoderFunction::FuelStatus => todo!(),
            DecoderFunction::Percent => todo!(),
            DecoderFunction::PercentCentered => todo!(),
            DecoderFunction::FuelPressure => todo!(),
            DecoderFunction::Pressure => todo!(),
            DecoderFunction::UAS(_) => todo!(),
            DecoderFunction::TimingAdvance => todo!(),
            DecoderFunction::AirStatus => todo!(),
            DecoderFunction::O2Sensors => todo!(),
            DecoderFunction::SensorVoltage => todo!(),
            DecoderFunction::OBdCompliance => todo!(),
            DecoderFunction::O2SensorsAlt => todo!(),
            DecoderFunction::AuxInputStatus => todo!(),
            DecoderFunction::SensorVoltageBig => todo!(),
            DecoderFunction::CurrentCentered => todo!(),
            DecoderFunction::EvapPressure => todo!(),
            DecoderFunction::Load => todo!(),
            DecoderFunction::FuelSystem => todo!(),
            // ... other cases ...
    }
}

// Implement HashMap lookup
pub fn get_command(name: &str) -> Option<&'static OBDCommand> {
    for command in MODE1.iter() {
        if command.name == name {
            return Some(command);
        }
    }

    println!("Command not found");
    None
}

fn decode_pid(data: &[u8]) -> Vec<u8> {
    // This can be more complicated based on how you interpret the data
    data.to_vec()
}

fn decode_status(data: &[u8]) -> String {
    // Example: interpret the first byte for MIL (Malfunction Indicator Lamp) status
    let mil_on = (data[0] & 0x80) != 0;
    let dtc_count = data[0] & 0x7F;
    format!("MIL: {}, DTC Count: {}", if mil_on { "ON" } else { "OFF" }, dtc_count)
}

fn decode_fuel_system(data: &[u8]) -> String {
    // Interpret based on fuel system status tables; this is a basic example
    match data[0] {
        0x01 => "Open loop due to insufficient engine temperature",
        0x02 => "Closed loop, using oxygen sensor feedback",
        // ... other cases ...
        _ => "Unknown",
    }.to_string()
}


fn decode_load(data: &[u8]) -> f32 {
    (data[0] as f32) * 100.0 / 255.0  // formula based on standard OBD-II interpretation
}


fn decode_temperature(data: &[u8]) -> i32 {
    data[0] as i32 - 40  // Celsius
}

fn decode_o2_voltage(data: &[u8]) -> f32 {
    (data[0] as f32) * 0.005  // typical formula for some O2 sensors
}

}