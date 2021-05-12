from tkinter import *

class ScrolledCanvas(Frame):
    def __init__(self, parent=None, color='brown'):
        Frame.__init__(self, parent)
        # self.pack()
        self.pack(expand=YES, fill=BOTH)
        canvas = Canvas(self, bg=color, relief=SUNKEN)
        canvas.config(width=300, height=200)
        canvas.config(scrollregion=(0, 0, 300, 1000))
        canvas.config(highlightthickness=0)
        # canvas.pack()

        sbar = Scrollbar(self)
        sbar.config(command=canvas.yview)
        canvas.config(yscrollcommand=sbar.set)
        sbar.pack(side=RIGHT, fill=Y)
        canvas.pack(side=LEFT, expand=YES, fill=BOTH)

        self.fillContent(canvas)
        canvas.bind('<Double-1>', self.onDoubleClick)

        self.canvas = canvas

    def fillContent(self, canvas):
        for i in range(10):
            canvas.create_text(150, 50 + (i * 100), text='span' + str(i), fill='beige')

    def onDoubleClick(self, event):
        print(event.x, event.y)
        print(self.canvas.canvasx(event.x), self.canvas.canvasy(event.y))

if __name__ == '__main__':
    ScrolledCanvas().mainloop()
