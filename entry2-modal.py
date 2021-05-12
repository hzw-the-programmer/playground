from tkinter import *
from entry2 import fields, makeform, fetch

def show(entries, popup):
    fetch(entries)
    popup.destroy()

def ask():
    popup = Toplevel()
    entries = makeform(popup, fields)
    Button(popup, text='OK', command=(lambda: show(entries, popup))).pack()
    popup.grab_set()
    popup.focus_set()
    popup.wait_window()

root = Tk()
Button(root, text='Dialog', command=ask).pack()
root.mainloop()
