import os, sys
from tkinter import *

imgdir = 'images'
imgfile = 'msn.gif'
if len(sys.argv) > 1:
    imgfile = sys.argv[1]
imgpath = os.path.join(imgdir, imgfile)

win = Tk()
imgobj = PhotoImage(file=imgpath)
canvas = Canvas(win)
canvas.pack(fill=BOTH)
canvas.config(width=imgobj.width(), height=imgobj.height())
canvas.create_image(20, 20, image=imgobj, anchor=NW)
win.mainloop()
