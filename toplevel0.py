from tkinter import *

win1 = Toplevel()
win2 = Toplevel()

Button(win1, text='win1', command=sys.exit).pack()
# Button(win2, text='win2', command=sys.exit).pack()
Button(win2, text='win2', command=win1.destroy).pack()

Label(text='main').pack()

win1.mainloop()
