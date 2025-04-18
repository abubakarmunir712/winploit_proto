import socket
import sys

def main():
    # Create a TCP socket
    client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    # Connect to the server
    host = '127.0.0.1'
    port = 7878
    try:
        client_socket.connect((host, port))
    except ConnectionRefusedError:
        print("Error: Could not connect to server. Is the Rust server running?")
        return

    print(f"Connected to server at {host}:{port}")
    print("Type 'exit' to quit")

    while True:
        # Get command from user
        command = input(f"{host}:{port} $ ")
        
        # Send command to server
        client_socket.sendall(command.encode('utf-8'))
        
        # Check for exit command
        if command.strip() == "exit":
            data = client_socket.recv(1024)
            print(data.decode('utf-8').strip())
            break

        # Receive and print response
        data = client_socket.recv(1024)
        print(data.decode('utf-8').strip())

    # Close the socket
    client_socket.close()

if __name__ == "__main__":
    main()