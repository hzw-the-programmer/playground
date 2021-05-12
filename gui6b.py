from sys import exit
from tkinter import *
from gui6 import Hello

parent = Frame()
parent.pack()

Hello(parent).pack(side=RIGHT)
# Hello(parent)
Button(parent, text='Attach', command=exit).pack(side=LEFT)

parent.mainloop()
