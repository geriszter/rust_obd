import socket

class Elm327Emulator:
    def __init__(self):
        self.headers_enabled = True
        self.elm327_commands = {
            "ATZ": "ELM327 v1.5",          # Reset command, returns version number
            "ATE0": "OK",                 # Echo off
            "ATE1": "OK",                 # Echo on
            "ATL0": "OK",                 # Linefeeds off
            "ATL1": "OK",                 # Linefeeds on
            "ATSP0": "OK",                # Set protocol to auto
            "ATH0": "OK",                 # Headers off
            "ATH1": "OK",                 # Headers on
            "0100": "41 00 BE 7F B8 10",  # Request supported PIDs 01-20
            "0105": "41 05 82",           # Engine coolant temperature
            "010C": "41 0C 0A 94",        # Engine RPM (e.g. 0A 94 represents 2700 RPM. Calculation: (0A94h = 2708) / 4 = 677 RPM)
            "010D": "41 0D 56",           # Vehicle speed in km/h
            "0111": "41 11 64",           # Throttle position percentage
            # ... add more commands and responses as needed
        }

    def handle_elm_command(self, command):
        print(f"Received Command: {command}")
        response = self.elm327_commands.get(command, "UNKNOWN COMMAND")
        if command == "ATH0":
            self.headers_enabled = False
        elif command == "ATH1":
            self.headers_enabled = True
        elif command.startswith("01") and not self.headers_enabled:
            response = response[6:]
            
        response = response + ">"
        return response

    def run(self):
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind(('127.0.0.1', 35000))
            s.listen()
            print("Waiting for a connection...")
            conn, addr = s.accept()
            with conn:
                print('Connected by', addr)
                while True:
                    data = conn.recv(1024)
                    if not data:
                        break
                    response = self.handle_elm_command(data.decode('utf-8').strip())
                    response = response.encode('utf-8')
                    print("Send:", response)
                    conn.sendall(response)


if __name__ == "__main__":
    emulator = Elm327Emulator()
    emulator.run()
