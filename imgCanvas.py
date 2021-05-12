import os, sys
from tkinter import *

imgdir = 'images'
imgfile = 'msn.gif'
if len(sys.argv) > 1:
    imgfile = sys.argv[1]
imgpath = os.path.join(imgdir, imgfile)

win = Tk()
img = PhotoImage(file=imgpath)
canvas = Canvas(win, bg='beige')
canvas.pack(fill=BOTH)
canvas.create_image(2, 2, image=img, anchor=NW)
win.mainloop()
