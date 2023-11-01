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
    VIN,
    Status,
    FuelStatus,
    Temperature,
    FuelPressure,
    IntakePressure,
    TimingAdvance,
    AirStatus,
    O2Sensor,
    AuxInputStatus,
    EvapPressure,
    Load,
    RPM,
    MAF,
    Speed,
    FuelSystem,
    FuelTrim,
    ThrottlePosition,
    Runtime,
    FuelLevel,
    BarometricPressure,
    ModuleVoltage,
    AbsoluteLoad,
    EquivRatio,
    CatalystTemperature,
    SecondaryAirStatus,
    OxygenSensorsPresent,
    DistanceTraveledMILON,
    EgreError,
    FuelRailPressure,
    EvapPurge,
    WarmUps,
    DistanceTraveled,
    EvapVaporPressureAlt,
    FuelRailPressureAbs,
    OilTemperature,
    InjectionTiming,
    FuelRate,
    EthanolFuelPercent,
    AbsoluteEvapPressure,
    O2SensorCurrent,
    BatteryVoltage,
    AirFlowRate,
    ExhaustPressure,
    HybridBatteryRemaining,
    // ...
}

//there are 78 commands
static MODE1: [OBDCommand; 78] = [
    OBDCommand { name: "PIDS_A", description: "Supported PIDs [01-20]", cmd: b"0100", bytes: 6, decoder: DecoderFunction::PID },
    OBDCommand { name: "STATUS", description: "Status since DTCs cleared", cmd: b"0101", bytes: 6, decoder: DecoderFunction::Status },
    OBDCommand { name: "FREEZE_DTC", description: "Freeze DTC", cmd: b"0102", bytes: 2, decoder: DecoderFunction::Default },
    OBDCommand { name: "FUEL_SYSTEM", description: "Fuel system status", cmd: b"0103", bytes: 2, decoder: DecoderFunction::FuelSystem },
    OBDCommand { name: "ENGINE_LOAD", description: "Calculated engine load", cmd: b"0104", bytes: 1, decoder: DecoderFunction::Load },
    OBDCommand { name: "ENGINE_COOOLANT_TEMPERATURE", description: "Engine coolant temperature", cmd: b"0105", bytes: 1, decoder: DecoderFunction::Temperature },

    OBDCommand { name: "SHORT_TERM_FUEL_TRIM_BANK1", description: "Short term fuel % trim - Bank 1", cmd: b"0106", bytes: 1, decoder: DecoderFunction::FuelTrim },
    OBDCommand { name: "LONG_TERM_FUEL_TRIM_BANK1", description: "Long term fuel % trim - Bank 1", cmd: b"0107", bytes: 1, decoder: DecoderFunction::FuelTrim },
    OBDCommand { name: "SHORT_TERM_FUEL_TRIM_BANK2", description: "Short term fuel % trim - Bank 2", cmd: b"0108", bytes: 1, decoder: DecoderFunction::FuelTrim },
    OBDCommand { name: "LONG_TERM_FUEL_TRIM_BANK2", description: "Long term fuel % trim - Bank 2", cmd: b"0109", bytes: 1, decoder: DecoderFunction::FuelTrim },
    OBDCommand { name: "FUEL_PRESSURE", description: "Fuel pressure", cmd: b"010A", bytes: 1, decoder: DecoderFunction::FuelPressure },
    OBDCommand { name: "INTAKE_MANIFOLD_PRESSURE", description: "Intake manifold absolute pressure", cmd: b"010B", bytes: 1, decoder: DecoderFunction::IntakePressure },
    OBDCommand { name: "RPM", description: "Engine RPM", cmd: b"010C", bytes: 2, decoder: DecoderFunction::RPM },
    OBDCommand { name: "SPEED", description: "Vehicle speed", cmd: b"010D", bytes: 1, decoder: DecoderFunction::Speed },
    OBDCommand { name: "TIMING_ADVANCE", description: "Timing advance", cmd: b"010E", bytes: 1, decoder: DecoderFunction::TimingAdvance },
    OBDCommand { name: "INTAKE_AIR_TEMPERATURE", description: "Intake air temperature", cmd: b"010F", bytes: 1, decoder: DecoderFunction::Temperature },
    OBDCommand { name: "MAF", description: "Mass air flow rate", cmd: b"0110", bytes: 2, decoder: DecoderFunction::MAF },
    OBDCommand { name: "THROTTLE_POSITION", description: "Throttle position", cmd: b"0111", bytes: 1, decoder: DecoderFunction::ThrottlePosition },

    OBDCommand { name: "O2_SENSOR_BANK1_SENSOR1", description: "O2 Sensor Bank 1, Sensor 1", cmd: b"0114", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK1_SENSOR2", description: "O2 Sensor Bank 1, Sensor 2", cmd: b"0115", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK1_SENSOR3", description: "O2 Sensor Bank 1, Sensor 3", cmd: b"0116", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK1_SENSOR4", description: "O2 Sensor Bank 1, Sensor 4", cmd: b"0117", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK2_SENSOR1", description: "O2 Sensor Bank 2, Sensor 1", cmd: b"0118", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK2_SENSOR2", description: "O2 Sensor Bank 2, Sensor 2", cmd: b"0119", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK2_SENSOR3", description: "O2 Sensor Bank 2, Sensor 3", cmd: b"011A", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK2_SENSOR4", description: "O2 Sensor Bank 2, Sensor 4", cmd: b"011B", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "RUNTIME_SINCE_ENGINE_START", description: "Run time since engine start", cmd: b"011F", bytes: 2, decoder: DecoderFunction::Runtime },
    OBDCommand { name: "FUEL_TANK_LEVEL_INPUT", description: "Fuel Tank Level Input", cmd: b"012F", bytes: 1, decoder: DecoderFunction::FuelLevel },
    OBDCommand { name: "EVAP_SYSTEM_VAPOR_PRESSURE", description: "Evap System Vapor Pressure", cmd: b"0130", bytes: 2, decoder: DecoderFunction::EvapPressure },
    OBDCommand { name: "BAROMETRIC_PRESSURE", description: "Barometric Pressure", cmd: b"0133", bytes: 1, decoder: DecoderFunction::BarometricPressure },

    OBDCommand { name: "CONTROL_MODULE_VOLTAGE", description: "Control module voltage", cmd: b"0142", bytes: 2, decoder: DecoderFunction::ModuleVoltage },
    OBDCommand { name: "ABSOLUTE_LOAD_VALUE", description: "Absolute load value", cmd: b"0143", bytes: 2, decoder: DecoderFunction::AbsoluteLoad },
    OBDCommand { name: "FUEL_AIR_COMMANDED_EQUIV_RATIO", description: "Commanded equivalence ratio", cmd: b"0144", bytes: 2, decoder: DecoderFunction::EquivRatio },
    OBDCommand { name: "RELATIVE_THROTTLE_POSITION", description: "Relative throttle position", cmd: b"0145", bytes: 1, decoder: DecoderFunction::ThrottlePosition},
    OBDCommand { name: "AMBIENT_AIR_TEMPERATURE", description: "Ambient air temperature", cmd: b"0146", bytes: 1, decoder: DecoderFunction::Temperature },
    OBDCommand { name: "CATALYST_TEMPERATURE_BANK1_SENSOR1", description: "Catalyst Temperature: Bank 1, Sensor 1", cmd: b"013C", bytes: 2, decoder: DecoderFunction::CatalystTemperature },
    OBDCommand { name: "CATALYST_TEMPERATURE_BANK2_SENSOR1", description: "Catalyst Temperature: Bank 2, Sensor 1", cmd: b"013D", bytes: 2, decoder: DecoderFunction::CatalystTemperature },
    OBDCommand { name: "CATALYST_TEMPERATURE_BANK1_SENSOR2", description: "Catalyst Temperature: Bank 1, Sensor 2", cmd: b"013E", bytes: 2, decoder: DecoderFunction::CatalystTemperature },
    OBDCommand { name: "CATALYST_TEMPERATURE_BANK2_SENSOR2", description: "Catalyst Temperature: Bank 2, Sensor 2", cmd: b"013F", bytes: 2, decoder: DecoderFunction::CatalystTemperature },
    OBDCommand { name: "SECONDARY_AIR_STATUS", description: "Secondary air status", cmd: b"012C", bytes: 1, decoder: DecoderFunction::SecondaryAirStatus },
    OBDCommand { name: "OXYGEN_SENSORS_PRESENT", description: "Oxygen sensors present", cmd: b"0113", bytes: 2, decoder: DecoderFunction::OxygenSensorsPresent },
    OBDCommand { name: "DISTANCE_TRAVELED_MIL_ON", description: "Distance traveled with malfunction indicator lamp (MIL) on", cmd: b"0121", bytes: 2, decoder: DecoderFunction::DistanceTraveledMILON },
    OBDCommand { name: "EGR_ERROR", description: "EGR Error", cmd: b"0158", bytes: 1, decoder: DecoderFunction::EgreError },
    OBDCommand { name: "FUEL_RAIL_PRESSURE", description: "Fuel Rail Pressure", cmd: b"0123", bytes: 2, decoder: DecoderFunction::FuelRailPressure },

    OBDCommand { name: "EVAP_SYSTEM_PURGE", description: "Commanded Evaporative Purge", cmd: b"011E", bytes: 1, decoder: DecoderFunction::EvapPurge },
    OBDCommand { name: "FUEL_LEVEL", description: "Fuel Level Input", cmd: b"012F", bytes: 1, decoder: DecoderFunction::FuelLevel },
    OBDCommand { name: "WARM_UPS_SINCE_DTC_CLEAR", description: "Number of warm-ups since DTCs cleared", cmd: b"0130", bytes: 1, decoder: DecoderFunction::WarmUps },
    OBDCommand { name: "DISTANCE_SINCE_DTC_CLEAR", description: "Distance traveled since DTCs cleared", cmd: b"0131", bytes: 2, decoder: DecoderFunction::DistanceTraveled },
    OBDCommand { name: "EVAP_VAPOR_PRESSURE_ALT", description: "Evap. System Vapor Pressure", cmd: b"0146", bytes: 2, decoder: DecoderFunction::EvapVaporPressureAlt },
    OBDCommand { name: "FUEL_RAIL_PRESSURE_ABS", description: "Fuel Rail Absolute Pressure", cmd: b"0159", bytes: 2, decoder: DecoderFunction::FuelRailPressureAbs },
    OBDCommand { name: "ENGINE_OIL_TEMPERATURE", description: "Engine oil temperature", cmd: b"015C", bytes: 1, decoder: DecoderFunction::OilTemperature },
    OBDCommand { name: "FUEL_INJECTION_TIMING", description: "Fuel injection timing", cmd: b"015D", bytes: 2, decoder: DecoderFunction::InjectionTiming },
    OBDCommand { name: "FUEL_RATE", description: "Engine fuel rate", cmd: b"015E", bytes: 2, decoder: DecoderFunction::FuelRate },
    OBDCommand { name: "VEHICLE_IDENTIFICATION", description: "Vehicle identification number (VIN)", cmd: b"0159", bytes: 20, decoder: DecoderFunction::VIN },

    OBDCommand { name: "ETHANOL_FUEL", description: "Ethanol fuel %", cmd: b"0152", bytes: 1, decoder: DecoderFunction::EthanolFuelPercent },
    OBDCommand { name: "ABSOLUTE_EVAP_SYSTEM_VAPOR_PRESSURE", description: "Absolute Evap System Vapor Pressure", cmd: b"0153", bytes: 2, decoder: DecoderFunction::AbsoluteEvapPressure },
    OBDCommand { name: "EVAP_SYSTEM_VAPOR_PRESSURE2", description: "Evap System Vapor Pressure 2", cmd: b"0154", bytes: 2, decoder: DecoderFunction::EvapPressure },
    OBDCommand { name: "SECONDARY_AIR_STATUS2", description: "Secondary air status 2", cmd: b"0155", bytes: 1, decoder: DecoderFunction::SecondaryAirStatus },
    OBDCommand { name: "O2_SENSOR_CURRENT_BANK1_SENSOR1", description: "O2 Sensor Current: Bank 1, Sensor 1", cmd: b"0156", bytes: 2, decoder: DecoderFunction::O2SensorCurrent },
    OBDCommand { name: "O2_SENSOR_CURRENT_BANK1_SENSOR2", description: "O2 Sensor Current: Bank 1, Sensor 2", cmd: b"0157", bytes: 2, decoder: DecoderFunction::O2SensorCurrent },
    OBDCommand { name: "CATALYST_TEMPERATURE2", description: "Catalyst Temperature 2", cmd: b"0160", bytes: 2, decoder: DecoderFunction::CatalystTemperature },
    OBDCommand { name: "BATTERY_VOLTAGE", description: "Battery Voltage", cmd: b"0161", bytes: 2, decoder: DecoderFunction::BatteryVoltage },
    OBDCommand { name: "AIR_FLOW_RATE", description: "Air Flow Rate from MAF", cmd: b"0162", bytes: 2, decoder: DecoderFunction::AirFlowRate },
    OBDCommand { name: "THROTTLE_PEDAL", description: "Throttle Pedal Position", cmd: b"0163", bytes: 1, decoder: DecoderFunction::ThrottlePosition},
    OBDCommand { name: "RUN_TIME", description: "Engine Run Time", cmd: b"0164", bytes: 2, decoder: DecoderFunction::Runtime },
    OBDCommand { name: "EGR_TEMPERATURE", description: "EGR Temperature", cmd: b"0165", bytes: 1, decoder: DecoderFunction::Temperature },
    OBDCommand { name: "FUEL_PRESSURE2", description: "Fuel Pressure 2", cmd: b"0166", bytes: 1, decoder: DecoderFunction::FuelPressure },
    OBDCommand { name: "INTAKE_AIR_TEMPERATURE2", description: "Intake air temperature 2", cmd: b"0167", bytes: 1, decoder: DecoderFunction::Temperature },
    OBDCommand { name: "AIR_STATUS", description: "Commanded Air Status", cmd: b"0168", bytes: 1, decoder: DecoderFunction::AirStatus },
    OBDCommand { name: "OXYGEN_SENSORS_PRESENT2", description: "Oxygen sensors present 2", cmd: b"0169", bytes: 2, decoder: DecoderFunction::O2Sensor},
    OBDCommand { name: "O2_SENSOR_BANK3_SENSOR1", description: "O2 Sensor Bank 3, Sensor 1", cmd: b"016A", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "O2_SENSOR_BANK3_SENSOR2", description: "O2 Sensor Bank 3, Sensor 2", cmd: b"016B", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "OXYGEN_SENSORS_VOLTAGE", description: "Oxygen sensors voltage", cmd: b"016C", bytes: 2, decoder: DecoderFunction::O2Sensor },
    OBDCommand { name: "FUEL_SYSTEM_STATUS", description: "Fuel system status", cmd: b"016D", bytes: 2, decoder: DecoderFunction::FuelStatus },
    OBDCommand { name: "EXHAUST_PRESSURE", description: "Exhaust Gas Pressure", cmd: b"016E", bytes: 1, decoder: DecoderFunction::ExhaustPressure },
    OBDCommand { name: "FUEL_RATE2", description: "Engine fuel rate 2", cmd: b"0170", bytes: 2, decoder: DecoderFunction::FuelRate },
    OBDCommand { name: "AUX_INPUT_STATUS", description: "Auxiliary input status (power take off)" , cmd: b"011E", bytes: 3, decoder: DecoderFunction::AuxInputStatus},
    OBDCommand { name: "HYBRID_BATTERY_REMAINING", description: "Hybrid battery pack remaining life", cmd: b"015B", bytes: 3, decoder: DecoderFunction::HybridBatteryRemaining }


];

impl OBDCommand {
    
    
    pub fn decode_data(command: &OBDCommand, raw_data: &[u8]) -> String {
        match command.decoder {
            DecoderFunction::Default => raw_data.iter().map(|&byte| format!("{:02X}", byte)).collect::<String>(), //So if you had raw data like [0x12, 0x34, 0x56], it would be converted to the string "123456".
            DecoderFunction::PID => Self::decode_pids(raw_data),
            DecoderFunction::Status => Self::decode_status(raw_data),
            DecoderFunction::Temperature => Self::decode_temperature(raw_data).to_string(),
            DecoderFunction::FuelStatus => Self::decode_fuel_system(raw_data),
            DecoderFunction::FuelPressure => Self::decode_fuel_pressure(raw_data).to_string(),
            DecoderFunction::IntakePressure => Self::decode_intake_pressure(raw_data).to_string(),
            DecoderFunction::TimingAdvance => Self::decode_timing_advance(raw_data).to_string(),
            DecoderFunction::O2Sensor => Self::decode_o2_sensor(raw_data),
            DecoderFunction::AuxInputStatus => Self::decode_aux_input_status(raw_data),
            DecoderFunction::EvapPressure => Self::decode_evap_vapor_pressure(raw_data).to_string(),
            DecoderFunction::Load => Self::decode_load(raw_data).to_string(),
            DecoderFunction::RPM => Self::decode_rpm(raw_data).to_string(),
            DecoderFunction::Speed => Self::decode_speed(raw_data).to_string(),
            DecoderFunction::MAF => Self::decode_maf_rate(raw_data).to_string(),
            DecoderFunction::FuelTrim => Self::decode_fuel_trim(raw_data).to_string(),
            DecoderFunction::ThrottlePosition => Self::decode_throttle_position(raw_data).to_string(),
            DecoderFunction::Runtime => Self::decode_runtime(raw_data).to_string(),
            DecoderFunction::FuelLevel => Self::decode_fuel_level(raw_data).to_string(),
            DecoderFunction::BarometricPressure => Self::decode_barometric_pressure(raw_data).to_string(),
            DecoderFunction::ModuleVoltage => Self::decode_module_voltage(raw_data).to_string(),
            DecoderFunction::AbsoluteLoad => Self::decode_absolute_load(raw_data).to_string(),
            DecoderFunction::EquivRatio => Self::decode_equiv_ratio(raw_data).to_string(),
            DecoderFunction::CatalystTemperature => Self::decode_catalyst_temperature(raw_data).to_string(),
            DecoderFunction::SecondaryAirStatus => Self::decode_secondary_air_status(raw_data).to_string(),
            DecoderFunction::OxygenSensorsPresent => Self::decode_oxygen_sensors_present(raw_data),
            DecoderFunction::DistanceTraveledMILON => Self::decode_distance_traveled_mil_on(raw_data).to_string(),
            DecoderFunction::EgreError => Self::decode_egr_error(raw_data).to_string(),
            DecoderFunction::FuelRailPressure => Self::decode_fuel_rail_pressure(raw_data).to_string(),
            DecoderFunction::EvapPurge => Self::decode_evap_purge(raw_data).to_string(),
            DecoderFunction::WarmUps => Self::decode_warmups(raw_data).to_string(),
            DecoderFunction::DistanceTraveled => Self::decode_distance_traveled(raw_data).to_string(),
            DecoderFunction::EvapVaporPressureAlt => Self::decode_evap_vapor_pressure(raw_data).to_string(),
            DecoderFunction::FuelRailPressureAbs => Self::decode_fuel_rail_pressure_abs(raw_data).to_string(),
            DecoderFunction::OilTemperature => Self::decode_temperature(raw_data).to_string(),
            DecoderFunction::InjectionTiming => Self::decode_injection_timing(raw_data).to_string(),
            DecoderFunction::FuelRate => Self::decode_fuel_rate(raw_data).to_string(),
            DecoderFunction::VIN => Self::decode_vin(raw_data),
            DecoderFunction::EthanolFuelPercent => Self::percent(raw_data).to_string(),
            DecoderFunction::AbsoluteEvapPressure => Self::decode_evap_vapor_pressure(raw_data).to_string(),
            DecoderFunction::O2SensorCurrent => Self::percent_centered(raw_data).to_string(),
            DecoderFunction::BatteryVoltage => Self::decode_battery_voltage(raw_data).to_string(),
            DecoderFunction::AirFlowRate => Self::decode_maf_rate(raw_data).to_string(),
            DecoderFunction::ExhaustPressure => Self::decode_exhaust_pressure(raw_data).to_string(),
            DecoderFunction::HybridBatteryRemaining => Self::percent(raw_data).to_string(),
            DecoderFunction::AirStatus => todo!(),
            DecoderFunction::FuelSystem => Self::decode_fuel_system(raw_data).to_string(),
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


pub fn get_command_info(name: &str) {
    for command in MODE1.iter() {
        if command.name == name {
            println!("Command Description: {}", command.description);
            println!("Byte size: {}", command.bytes);
            return;
        }
    }
    println!("Command not found");
}

fn decode_pids(data: &[u8]) -> String {
    let mut supported = Vec::new();
    
    for (byte_index, byte) in data.iter().enumerate() {
        for bit_index in 0..8 {
            if byte & (1 << bit_index) != 0 {
                supported.push(((byte_index * 8) + bit_index + 1) as u8);
            }
        }
    }

    // Convert Vec<u8> to a String of concatenated hex PIDs
    supported.iter().map(|pid| format!("{:02X}", pid)).collect::<String>()
}

fn decode_status(data: &[u8]) -> String {
    // Example: interpret the first byte for MIL (Malfunction Indicator Lamp) status
    let mil_on = (data[0] & 0x80) != 0;
    let dtc_count = data[0] & 0x7F;
    format!("MIL: {}, DTC Count: {}", if mil_on { "ON" } else { "OFF" }, dtc_count)
}

fn decode_fuel_system(data: &[u8]) -> String {
    let system1_status = match data[0] {
        0x01 => "Open loop due to insufficient engine temperature",
        0x02 => "Closed loop, using oxygen sensor feedback",
        0x04 => "Open loop due to engine load OR fuel cut due to deceleration",
        0x08 => "Open loop due to system failure",
        0x10 => "Closed loop, but fault in at least one oxygen sensor",
        _ => "Unknown status",
    };

    let system2_status = match data[1] {
        0x01 => "Open loop due to insufficient engine temperature",
        0x02 => "Closed loop, using oxygen sensor feedback",
        0x04 => "Open loop due to engine load OR fuel cut due to deceleration",
        0x08 => "Open loop due to system failure",
        0x10 => "Closed loop, but fault in at least one oxygen sensor",
        _ => "Unknown status",
    };

    format!("{}, {}", system1_status, system2_status)
}


fn decode_load(data: &[u8]) -> f32 {
    (data[0] as f32) * 100.0 / 255.0  // formula based on standard OBD-II interpretation
}


fn decode_temperature(data: &[u8]) -> f32 {
    (data[0] as f32) - 40.0  // Celsius
}

fn decode_fuel_pressure(data: &[u8]) -> f32 {
    data[0] as f32 * 3.0
}

fn decode_intake_pressure(data: &[u8]) -> f32 {
    data[0] as f32
}

fn decode_rpm(data: &[u8]) -> f32 {
    ((data[0] as f32) * 256.0 + (data[1] as f32)) / 4.0
}

fn decode_speed(data: &[u8]) -> f32 {
    (data[0] as f32) * 0.621371 // mph
}

fn decode_timing_advance(data: &[u8]) -> f32 {
    ((data[0] as f32) - 128.0) / 2.0
}

fn decode_maf_rate(data: &[u8]) -> f32 {
    ((data[0] as f32) * 256.0 + (data[1] as f32)) / 100.0 // g/s
}

fn decode_o2_sensor(data: &[u8]) -> String {
    let voltage = data[0] as f32 / 200.0;
    let fuel_to_air_ratio = data[1] as f32 / 128.0;
    format!("Voltage: {:.2} V, Fuel-to-Air Ratio: {:.2}", voltage, fuel_to_air_ratio) // need to split
}


fn decode_fuel_trim(data: &[u8]) -> f32 {
    (data[0] as f32 - 128.0) * 100.0 / 128.0 // Percentage
}

fn decode_runtime(data: &[u8]) -> u16 {
    // Combine two bytes into a 16-bit value to get the total runtime in seconds
    ((data[0] as u16) << 8) | data[1] as u16
}

fn decode_fuel_level(data: &[u8]) -> f32 {
    // Convert the byte value into a percentage
    data[0] as f32 / 2.55
}

fn decode_barometric_pressure(data: &[u8]) -> u8 {
    data[0] // kPa
}

fn decode_module_voltage(raw_data: &[u8]) -> f32 {
    let value = (raw_data[0] as u16) << 8 | (raw_data[1] as u16);
    value as f32 / 1000.0
}

fn decode_throttle_position(data: &[u8]) -> f32 {
    data[0] as f32 / 2.55 // Percentage
}

fn decode_absolute_load(raw_data: &[u8]) -> f32 {

    let value = (raw_data[0] as u16) << 8 | (raw_data[1] as u16);
    (value as f32 / 255.0) * 100.0
}

fn decode_equiv_ratio(raw_data: &[u8]) -> f32 {
    let value = (raw_data[0] as u16) << 8 | (raw_data[1] as u16);
    value as f32 / 32768.0
}

fn decode_catalyst_temperature(raw_data: &[u8]) -> f32 {
    let value = ((raw_data[0] as u16) << 8) | (raw_data[1] as u16);
    (value as f32 - 40.0) / 10.0
}

fn decode_secondary_air_status(raw_data: &[u8]) -> String {

    let status = match raw_data[0] {
        0x01 => "Upstream",
        0x02 => "Downstream of catalytic converter",
        0x04 => "From the outside atmosphere or off",
        0x08 => "Pump commanded on for diagnostics",
        _ => "Unknown",
    };

    status.to_string()
}

fn decode_oxygen_sensors_present(raw_data: &[u8]) -> String {
    let mut sensors = Vec::new();

    if raw_data[0] & 0x01 != 0 {
        sensors.push("Bank 1, Sensor 1");
    }
    if raw_data[0] & 0x02 != 0 {
        sensors.push("Bank 1, Sensor 2");
    }
    if raw_data[0] & 0x04 != 0 {
        sensors.push("Bank 1, Sensor 3");
    }
    if raw_data[0] & 0x08 != 0 {
        sensors.push("Bank 1, Sensor 4");
    }
    if raw_data[0] & 0x10 != 0 {
        sensors.push("Bank 2, Sensor 1");
    }
    if raw_data[0] & 0x20 != 0 {
        sensors.push("Bank 2, Sensor 2");
    }
    if raw_data[0] & 0x40 != 0 {
        sensors.push("Bank 2, Sensor 3");
    }
    if raw_data[0] & 0x80 != 0 {
        sensors.push("Bank 2, Sensor 4");
    }

    sensors.join(", ")
}

fn decode_distance_traveled_mil_on(raw_data: &[u8]) -> String {
    let distance_km = ((raw_data[0] as u16) << 8) | (raw_data[1] as u16);
    
    // Convert kilometers to miles
    let distance_miles = distance_km as f32 * 0.621371;

    distance_miles.to_string()
}

fn decode_egr_error(raw_data: &[u8]) -> String {

    // Calculate the EGR Error percentage
    let egr_error_percentage = ((raw_data[0] as i32 - 128) * 100) / 128;

    egr_error_percentage.to_string()
}

fn decode_fuel_rail_pressure(raw_data: &[u8]) -> f32 {
    // Calculate the Fuel Rail Pressure in kPa
    let pressure_kpa = (raw_data[0] as u16 * 256 + raw_data[1] as u16) as f32;
    return  pressure_kpa;
}

fn decode_evap_purge(data: &[u8]) -> f32 {

    let value = data[0] as f32;
    let purge_percentage = (value / 255.0) * 100.0;

    return purge_percentage;
}

fn decode_warmups(data: &[u8]) -> u8 {
    data[0]
}

fn decode_distance_traveled(data: &[u8]) -> f32 {
    let kilometers = ((data[0] as u16) * 256) + (data[1] as u16);
    kilometers as f32 * 0.621371 // miles
}

fn decode_evap_vapor_pressure(data: &[u8]) -> f32 {
    let raw_value = ((data[0] as i16) * 256) + (data[1] as i16);
    raw_value as f32 * 0.1
}

// scaling by 10 compare to normal pressure
fn decode_fuel_rail_pressure_abs(data: &[u8]) -> f32 {
    ((data[0] as f32) * 256.0 + (data[1] as f32)) * 10.0
}

fn decode_injection_timing(data: &[u8]) -> f32 {
    (((data[0] as f32) * 256.0 + (data[1] as f32)) / 128.0) - 210.0
}

fn decode_fuel_rate(data: &[u8]) -> f32 {
    ((data[0] as f32) * 256.0 + (data[1] as f32)) / 20.0 // l/h
}

fn decode_vin(data: &[u8]) -> String {
    // Convert bytes 1 to end into characters
    let vin_part: Vec<char> = data[1..].iter().map(|&byte| byte as char).collect();

    vin_part.into_iter().collect()
}

fn percent(data: &[u8]) -> f32 {
    (100.0 / 255.0) * data[0] as f32 //Percent
}

fn percent_centered(data: &[u8]) -> f32 {
    let v = data[0] as f32;
    (v - 128.0) * 100.0 / 128.0
}

fn decode_battery_voltage(data: &[u8]) -> f32 {
    ((data[0] as f32) * 256.0 + (data[1] as f32)) * 0.1 // volts
}

fn decode_exhaust_pressure(data: &[u8]) -> f32 {
    ((data[0] as f32) * 256.0 + (data[1] as f32)) * 0.1 // kPa
}

fn decode_aux_input_status(data: &[u8]) -> String {
    if data[0] & 0x01 == 0x01 {
        "Aux Input: ON".to_string()
    } else {
        "Aux Input: OFF".to_string()
    }
}

}