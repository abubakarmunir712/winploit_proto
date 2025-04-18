import socket
import sys
import struct

def read_exact(sock, n):
    """Read exactly n bytes from the socket."""
    data = b""
    while len(data) < n:
        packet = sock.recv(n - len(data))
        if not packet:
            raise ConnectionError("Socket connection broken")
        data += packet
    return data

def handle_client(client_socket, client_address):
    print(f"Accepted connection from {client_address[0]}:{client_address[1]}")
    
    # Read initial "ready" message
    try:
        len_bytes = read_exact(client_socket, 4)
        length = struct.unpack('!I', len_bytes)[0]
        data = read_exact(client_socket, length)
        print(f"Client message: {data.decode('utf-8').strip()}")
    except Exception as e:
        print(f"Error reading initial message: {e}")
        client_socket.close()
        return

    print("Type commands to send to the client (type 'exit' to quit)")

    while True:
        # Get command from user
        command = input(f"Command to {client_address[0]}:{client_address[1]} $ ")
        
        # Send command to client with newline
        client_socket.sendall(f"{command}\n".encode('utf-8'))
        
        # Check for exit command
        if command.strip() == "exit":
            len_bytes = read_exact(client_socket, 4)
            length = struct.unpack('!I', len_bytes)[0]
            data = read_exact(client_socket, length)
            print(data.decode('utf-8').strip())
            break

        # Read response
        len_bytes = read_exact(client_socket, 4)
        length = struct.unpack('!I', len_bytes)[0]
        data = read_exact(client_socket, length)
        if not data:
            print(f"Client {client_address[0]}:{client_address[1]} disconnected")
            break
        # Debug: Log raw bytes for first command
        if command.strip() == "dir":
            print(f"Raw bytes for 'dir': {data[:100]}...")
        try:
            print(data.decode('utf-8').strip())
        except UnicodeDecodeError:
            print(f"Error decoding response: {data[:100]}...")

    client_socket.close()
    print(f"Connection with {client_address[0]}:{client_address[1]} closed")

def main():
    server_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    
    host = '0.0.0.0'
    port = 7878
    try:
        server_socket.bind((host, port))
    except OSError as e:
        print(f"Error: Could not bind to {host}:{port}. {e}")
        return

    server_socket.listen(1)
    print(f"Server listening on {host}:{port}")
    print("Waiting for a client connection...")

    try:
        client_socket, client_address = server_socket.accept()
        handle_client(client_socket, client_address)
    except KeyboardInterrupt:
        print("\nServer shutting down")
    except Exception as e:
        print(f"Error accepting connection: {e}")
    finally:
        server_socket.close()

if __name__ == "__main__":
    main()