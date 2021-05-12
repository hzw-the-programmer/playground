from tkinter import *
from dialogTable import demos
from quitter import Quitter

class Demo(Frame):
    def __init__(self, parent=None, **options):
        Frame.__init__(self, parent, **options)
        self.pack()
        Label(self, text='Radio demos').pack(side=TOP)
        self.var = StringVar()
        for key in demos:
            Radiobutton(self, text=key + '1', value=key, variable=self.var, command=self.onPress).pack(anchor=NW)
        self.var.set(key)
        Button(self, text='State', command=self.report).pack(fill=X)
        Quitter(self).pack(fill=X)

    def onPress(self):
        pick = self.var.get()
        print('You pressed', pick)
        print('result:', demos[pick]())

    def report(self):
        print(self.var.get())

if __name__ == '__main__':
    Demo().mainloop()
