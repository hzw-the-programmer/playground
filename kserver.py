from binascii import hexlify
import queue
import socket
import threading
from tkinter import *

from k import K
from utils import unpack_header, unpack_51_req, type_to_str

LOCAL_ADDRESS = ('', 19268)

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
sock.bind(LOCAL_ADDRESS)

rq = queue.Queue()

def rtt():
    while True:
        rq.put(sock.recvfrom(1024))

rt = threading.Thread(target=rtt)
rt.daemon = True
rt.start()

class Grid(Frame):
    def __init__(self, parent=None):
        Frame.__init__(self, parent)
        self.pack(expand=YES, fill=BOTH)
        self.row = 0
        self.column = 0

    def rfrq(self):
        try:
            (pkt, address) = rq.get(block=False)
        except queue.Empty:
            pass
        else:
            (ip, port) = address
            (valid, protocal, length, sn, id, dt, cmd, data) = unpack_header(pkt)
            (sn, result, data) = unpack_51_req(data)

            k = K(self)
            k.grid(row=self.row, column=self.column)

            k.sn.config(text='sn: {0}'.format(sn))
            k.ip.config(text='ip: {0}'.format(ip))
            k.port.config(text='port: {0}'.format(port))
            k.protocal.config(text='protocal: 0x{0:x}'.format(protocal))
            maxrow = maxcolumn = 0
            for r in result:
                if r[0] > maxrow:
                    maxrow = r[0]
                if r[1] > maxcolumn:
                    maxcolumn = r[1]
            for i in range(maxrow + 1):
                lbl = Label(k.data)
                lbl.grid(row=i, column=0)
                if i != 0:
                    lbl.config(text=i)
            for r in result:
                lbl = Label(k.data)
                lbl.grid(row=r[0], column=r[1])
                lbl.config(text='0x{0:x}'.format(r[3]))

            self.column += 1
            if (self.column == 2):
                self.column = 0
                self.row += 1

        self.after(100, self.rfrq)

root = Tk()
root.title('kserver - by ZhiWen HE')
root.state('zoomed')
Grid(root).rfrq()
root.mainloop()
