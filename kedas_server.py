from binascii import hexlify
from socket import AF_INET
from socket import SOCK_DGRAM
from socket import socket
from threading import Thread
from utils import unpack_header

def read_pkt():
    while True:
        (data, address) = sock.recvfrom(1024)
        (data, valid, protocal, length, sn, id, time, cmd) = unpack_header(data)
        print('{0} {1} 0x{2:x} {3}'.format(sn, id, cmd, time))

address = ('', 19267)
sock = socket(AF_INET, SOCK_DGRAM)
sock.bind(address)

thread = Thread(target=read_pkt)
thread.start()
thread.join()
