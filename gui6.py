from tkinter import *

class Hello(Frame):
    def __init__(self, parent=None):
        Frame.__init__(self, parent)
        self.pack()
        self.make_widgets()
        self.data = 23

    def make_widgets(self):
        widget = Button(self, text='Hello frame world!', command=self.message)
        widget.pack(side=LEFT)

    def message(self):
        print('Hello frame world %s!' % self.data)
        self.data += 1

if __name__ == '__main__':
    Hello().mainloop()
