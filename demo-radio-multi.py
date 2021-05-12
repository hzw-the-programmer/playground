from tkinter import *

root = Tk()
var = StringVar()
for i in range(10):
    Radiobutton(root, text=str(i), value=str(i % 3), variable=var).pack(side=LEFT)
var.set(' ')
root.mainloop()
