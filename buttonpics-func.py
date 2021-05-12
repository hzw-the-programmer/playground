import glob
import os
import random
from tkinter import *
import demoCheck

imgdir = 'images'
imgfile = '*.gif'
imgpath = os.path.join(imgdir, imgfile)

def draw():
    file, photo = random.choice(images)
    lbl.config(text=file)
    btn.config(image=photo)

root = Tk()
lbl = Label(root, text='none', bg='blue', fg='red')
btn = Button(root, text='press me', command=draw, bg='white')
lbl.pack(fill=BOTH)
btn.pack(fill=BOTH)
demoCheck.Demo(root, relief=SUNKEN, bd=2).pack(fill=BOTH)

files = glob.glob(imgpath)
images = [(file, PhotoImage(file=file)) for file in files]
print(files)

root.mainloop()
