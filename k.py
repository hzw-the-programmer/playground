from tkinter import *

class K(Frame):
    def __init__(self, parent=None):
        Frame.__init__(self, parent)
        self.sn = Label(self)
        self.sn.grid(row=0, column=0, sticky=W)
        self.ip = Label(self)
        self.ip.grid(row=1, column=0, sticky=W)
        self.port = Label(self)
        self.port.grid(row=2, column=0, sticky=W)
        self.protocal = Label(self)
        self.protocal.grid(row=3, column=0, sticky=W)
        self.data = Frame(self)
        self.data.grid(row=4, column=0)
