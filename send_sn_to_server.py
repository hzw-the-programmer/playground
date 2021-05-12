import socket

IP = '127.0.0.1'
PORT = 0x4B44

CMD = 'R'

data = 'P2'

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.sendto(data, (IP, PORT))
