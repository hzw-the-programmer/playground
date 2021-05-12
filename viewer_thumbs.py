import os, sys, math
from tkinter import *
from PIL import Image
from PIL.ImageTk import PhotoImage

def makeThumbs(imgdir, size=(100, 100), thumbdir=None):
    if not thumbdir:
        thumbdir = os.path.join(imgdir, 'thumbs')
    if not os.path.exists(thumbdir):
        os.mkdir(thumbdir)

    thumbs = []
    for imgfile in os.listdir(imgdir):
        thumbpath = os.path.join(thumbdir, imgfile)
        if os.path.exists(thumbpath):
            thumbobj = Image.open(thumbpath)
            thumbs.append((imgfile, thumbobj))
        else:
            print('making', thumbpath)
            imgpath = os.path.join(imgdir, imgfile)
            try:
                imgobj = Image.open(imgpath)
                imgobj.thumbnail(size, Image.ANTIALIAS)
                imgobj.save(thumbpath)
                thumbs.append((imgfile, imgobj))
            except:
                print('Skipping', imgpath)

    return thumbs

class ViewOne(Toplevel):
    def __init__(self, imgdir, imgfile):
        Toplevel.__init__(self)
        self.title(imgfile)
        imgpath = os.path.join(imgdir, imgfile)
        photo = PhotoImage(file=imgpath)
        Label(self, image=photo).pack()
        self.savephoto = photo

def viewer(imgdir, kind=Toplevel, cols=None):
    win = kind()
    win.title('Viewer: ' + imgdir)

    Button(win, text='Quit', command=win.quit, bg='beige').pack(side=BOTTOM, fill=X)

    thumbs = makeThumbs(imgdir)
    if not cols:
        cols = int(math.ceil(math.sqrt(len(thumbs))))

    savedphotos = []
    while thumbs:
        thumbsrow, thumbs = thumbs[:cols], thumbs[cols:]
        row = Frame(win)
        row.pack(fill=BOTH)
        for imgfile, imgobj in thumbsrow:
            photo = PhotoImage(imgobj)
            link = Button(row, image=photo)
            handler = lambda savefile=imgfile: ViewOne(imgdir, savefile)
            link.config(command=handler)
            link.pack(side=LEFT, expand=YES)
            savedphotos.append(photo)

    return win, savedphotos

if __name__ == '__main__':
    # makeThumbs('images')
    # ViewOne('images', 'msn.gif').mainloop()
    imgdir = len(sys.argv) > 1 and sys.argv[1] or 'images'
    main, save = viewer(imgdir, kind=Tk)
    main.mainloop()
