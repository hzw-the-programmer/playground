from tkinter import *

root = Tk()
states = []
for i in range(10):
    var = IntVar()
    Checkbutton(root, text=str(i), variable=var).pack(side=LEFT)
    states.append(var)

root.mainloop()
print([var.get() for var in states])
