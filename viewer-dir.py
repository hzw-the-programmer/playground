import os, sys
from tkinter import *
from PIL.ImageTk import PhotoImage

imgdir = 'images'
if len(sys.argv) > 1:
    imgdir = sys.argv[1]

main = Tk()
main.title('Viewer')
Button(main, text='Quit', command=main.quit, font=('courier', 25)).pack()

savedphotos = []

imgfiles = os.listdir(imgdir)
for imgfile in imgfiles:
    win = Toplevel()
    win.title(imgfile)
    imgpath = os.path.join(imgdir, imgfile)
    try:
        photo = PhotoImage(file=imgpath)
        savedphotos.append(photo)
        Label(win, image=photo).pack()
    except:
        errmsg = 'skipping %s\n%s' % (imggile, sys.exc_info()[1])
        Label(win, text=errmsg).pack()

main.mainloop()
