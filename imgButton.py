import os, sys
from tkinter import *

imgdir = 'images'
imgfile = 'msn.gif'
if len(sys.argv) > 1:
    imgfile = sys.argv[1]
imgpath = os.path.join(imgdir, imgfile)

win = Tk()
imgobj = PhotoImage(file=imgpath)
Button(win, image=imgobj).pack()
win.mainloop()
