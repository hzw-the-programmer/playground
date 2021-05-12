from tkinter import *
from grid1 import colors

def gridbox(parent):
    row = 0
    for c in colors:
        Label(parent, text=c, relief=RIDGE, width=25).grid(row=row, column=0)
        ent = Entry(parent, bg=c, width=50)
        ent.grid(row=row, column=1)
        ent.insert(0, 'grid')
        row += 1

def packbox(parent):
    for c in colors:
        row = Frame(parent)
        row.pack()
        Label(row, text=c, width=25).pack(side=LEFT)
        ent = Entry(row, bg=c, relief=SUNKEN, width=50)
        ent.pack(side=RIGHT)
        ent.insert(0, 'pack')

if __name__ == '__main__':
    root = Tk()
    gridbox(Toplevel())
    packbox(Toplevel())
    Button(root, text='Quit', command=root.quit).pack()
    mainloop()
