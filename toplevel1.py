from tkinter import *

win1 = Tk()
win2 = Tk()

Button(win1, text='win1', command=win1.destroy).pack()
Button(win2, text='win2', command=win2.destroy).pack()

win1.mainloop()
