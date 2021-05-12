from tkinter import *
from quitter import Quitter

def fetch():
    print('Input => "%s"' % entry.get())

root = Tk()
entry = Entry(root)
entry.insert(0, 'Type words here')
entry.pack(side=TOP, fill=X)
entry.focus()
entry.bind('<Return>', (lambda event: fetch()))
btn = Button(root, text='Fetch', command=fetch)
btn.pack(side=LEFT)
Quitter(root).pack(side=RIGHT)
root.mainloop()
