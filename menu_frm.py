from tkinter import *
from tkinter.messagebox import showerror

def notdone():
    showerror('Not implemented', 'Not yet available')

def makemenu(parent):
    menubar = Frame(parent)
    menubar.pack(fill=X)

    fbtn = Menubutton(menubar, text='File', underline=0)
    fbtn.pack(side=LEFT)
    fmenu = Menu(fbtn)
    fbtn.config(menu=fmenu)

    fmenu.add_command(label='New...', command=notdone, underline=0)
    fmenu.add_command(label='Open...', command=notdone, underline=0)
    fmenu.add_command(label='Quit', command=parent.quit, underline=0)

    ebtn = Menubutton(menubar, text='Edit', underline=0)
    ebtn.pack(side=LEFT)
    emenu = Menu(ebtn, tearoff=False)
    ebtn.config(menu=emenu)

    emenu.add_command(label='Cut', command=notdone, underline=0)
    emenu.add_command(label='Paste', command=notdone, underline=0)
    emenu.add_separator()
    submenu = Menu(emenu)
    submenu.add_command(label='Spam', command=notdone, underline=0)
    submenu.add_command(label='Eggs', command=notdone, underline=0)
    emenu.add_cascade(label='Stuff', menu=submenu, underline=0)

    return menubar

if __name__ == '__main__':
    root = Tk()
    root.title('menu_frm')
    makemenu(root)
    msg = Label(root, text='Frame menu basics')
    msg.pack(expand=YES, fill=BOTH)
    msg.config(relief=SUNKEN, width=40, height=7, bg='beige')
    root.mainloop()
