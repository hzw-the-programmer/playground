import queue
import socket
import sys
import threading

local_ip = ""
local_port = int(sys.argv[1])
remote_ip = sys.argv[2]
remote_port = int(sys.argv[3])

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind((local_ip, local_port))

rq = queue.Queue()
wq = queue.Queue()

def read_pkt():
    while True:
        rq.put(sock.recvfrom(1024))

def write_pkt():
    while True:
        (string, address) = wq.get()
        sock.sendto(string, address)

read_thd = threading.Thread(target=read_pkt)
read_thd.start()

write_thd = threading.Thread(target=write_pkt)
write_thd.start()

while True:
    try:
        (string, address) = rq.get(False)
        print(address, string.decode('utf-8'))
    except:
        pass

    string = input("write:")
    wq.put((string.encode('utf-8'), (remote_ip, remote_port)))
