from binascii import hexlify
from socket import socket
from socket import SOCK_DGRAM
from threading import Thread
from time import sleep

def read():
    s = socket(type=SOCK_DGRAM)
    s.bind(('', 19267))
    while True:
        data, address = s.recvfrom(65565)
        print(address, hexlify(data))

t = Thread(target=read)
t.start()
t.join()

print('finished')
