from tkinter import *

def greeting():
    print('Hello stdout world!...')

win = Frame()
win.pack()

# Label(win, text='Hello container world').pack(side=TOP)
# Button(win, text='Hello', command=greeting).pack(side=LEFT)
# Label(win, text='Hello container world').pack(side=TOP)
# Button(win, text='Quit', command=win.quit).pack(side=RIGHT)
# Label(win, text='Hello container world').pack(side=TOP)

Button(win, text='Hello', command=greeting).pack(side=LEFT, fill=Y)
Label(win, text='Hello container world').pack(side=TOP)
# Button(win, text='Quit', command=win.quit).pack(side=RIGHT, fill=X)
# Button(win, text='Quit', command=win.quit).pack(side=RIGHT, expand=YES)
Button(win, text='Quit', command=win.quit).pack(side=RIGHT, expand=YES, fill=X)

win.mainloop()
