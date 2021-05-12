import glob, os, random
from tkinter import *
import demoCheck

imgdir = 'images'

class ButtonpicsDemo(Frame):
    def __init__(self, imgdir=imgdir, parent=None):
        Frame.__init__(self, parent)
        self.pack()
        self.lbl = Label(self, text='none', bg='blue', fg='red')
        self.btn = Button(self, text='press me', command=self.draw, bg='beige')
        # self.lbl.pack()
        self.lbl.pack(fill=BOTH)
        # self.btn.pack()
        # self.btn.pack(pady=10)
        # self.btn.pack(pady=10, fill=X)
        self.btn.pack(pady=10, fill=X, padx=10)
        # demoCheck.Demo(self, relief=SUNKEN, bd=2).pack()
        demoCheck.Demo(self, relief=SUNKEN, bd=2).pack(fill=BOTH)
        files = glob.glob(os.path.join(imgdir, '*.gif'))
        self.images = [(file, PhotoImage(file=file)) for file in files]
        print(files)

    def draw(self):
        file, photo = random.choice(self.images)
        self.lbl.config(text=file)
        self.btn.config(image=photo)

if __name__ == '__main__':
    ButtonpicsDemo().mainloop()
