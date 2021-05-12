from tkinter import *
from tkinter.messagebox import showerror

def notdone():
    showerror('Not implemented', 'Not yet available')

def makemenu(root):
    top = Menu(root)
    file = Menu(top)
    edit = Menu(top, tearoff=False)
    submenu = Menu(edit)

    root.config(menu=top)
    top.add_cascade(label='File', menu=file, underline=0)
    top.add_cascade(label='Edit', menu=edit, underline=0)

    file.add_command(label='New...', command=notdone, underline=0)
    file.add_command(label='Open...', command=notdone, underline=0)
    file.add_command(label='Quit...', command=notdone, underline=0)

    edit.add_command(label='Cut', command=notdone, underline=0)
    edit.add_command(label='Paste', command=notdone, underline=0)
    edit.add_separator()
    # submenu.add_command(label='Spam', command=root.quit, underline=0)
    submenu.add_command(label='Spam', command=root.destroy, underline=0)
    submenu.add_command(label='Eggs', command=notdone, underline=0)
    edit.add_cascade(label='Stuff', menu=submenu, underline=0)

if __name__ == '__main__':
    root = Tk()
    root.title('menu_win')
    makemenu(root)
    msg = Label(root, text='Window menu basics')
    msg.pack(expand=YES, fill=BOTH)
    msg.config(relief=SUNKEN, width=40, height=7, bg='beige')
    root.mainloop()
