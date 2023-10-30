# Rust OBD
<img src="/docs/logo.png" width="250" height="250">
This project is a basic implementation of a program to interact with OBD (On-Board Diagnostics) systems, specifically with ELM327 devices. It provides the ability to connect, send, and interpret OBD commands, with a focus on obtaining information about various car parameters.

## Features
- Connect to an ELM327 device over TCP/IP.
- Send and interpret basic OBD commands.
- Display the status and Diagnostic Trouble Codes (DTCs).
- Asynchronous implementation using Tokio.
- Extensible command interpretation framework.

## Usage
 - Ensure your ELM327 device is accessible at the address specified in the code (192.168.0.10:35000 by default).
 - Run the main Rust application.

Sample output:
```Rust
Successfully connected to ELM327 device!
ELM327 Version: [version]
Command Name: [command_name], Result: [command_result]
```
On errors, appropriate error messages will be displayed.

## Modules
- elm327: Contains the Elm327Connection struct which is responsible for establishing a connection to the ELM327 device, sending commands, and reading responses.
- command: Defines the OBDCommand struct and associated commands and decoding functions. This module contains the logic for encoding OBD-II commands, sending them to the ELM327 device, and decoding the received data.

## OBD-II Commands Reference
For a comprehensive list of OBD-II commands (PIDs), you can refer to the [Wikipedia page on OBD-II PIDs](https://en.wikipedia.org/wiki/OBD-II_PIDs).
