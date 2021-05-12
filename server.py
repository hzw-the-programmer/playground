from binascii import hexlify
from datetime import datetime
import os
from queue import Queue
import socket
from threading import Thread

from database import database
from utils import unpack_header, unpack_51

database.init()

ADDRESS = ('', 19268)

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(ADDRESS)

recv_que = Queue();
log_que = Queue();

def recv():
    while True:
        data, addr = sock.recvfrom(1024)
        recv_que.put((data, addr, datetime.now()))

def process_recv():
    while True:
        data, addr, time = recv_que.get()
        log_que.put(('received', (data, addr, time)))
        valid, protocal, length, sn, id, datetime, cmd, data = unpack_header(data)
        if valid:
            if cmd == 0x51:
                sn, result, data = unpack_51(data)
                database.enque((cmd, (sn, addr, datetime, result)))


def process_log():
    while True:
        action, data = log_que.get()
        if action == 'received':
            header = unpack_header(data[0])
            valid = header[0]
            sn = header[3]
            if valid:
                dir = 'log/{0}'.format(sn)
                try:
                    os.makedirs(dir)
                except Exception as e:
                    # print(e)
                    pass
                finally:
                    f = open('{0}/{1}-{2}-{3}'.format(dir, data[2].year, data[2].month, data[2].day), 'a')
                    f.write('{0} {1} {2} {3} {4}\n'.format(data[2], 'R', data[1][0], data[1][1], hexlify(data[0]).decode()))
                    f.close()

recv_thread = Thread(target=recv)
recv_thread.daemon = True
recv_thread.start()

process_recv_thread = Thread(target=process_recv)
process_recv_thread.daemon = True
process_recv_thread.start()

process_log_thread = Thread(target=process_log)
process_log_thread.daemon = True
process_log_thread.start()

input('Press Any Key To Stop...')
