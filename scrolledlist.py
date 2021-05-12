from tkinter import *

class ScrolledList(Frame):
    def __init__(self, options, parent=None):
        Frame.__init__(self, parent)
        self.pack(expand=YES, fill=BOTH)
        self.makewidgets(options)

    def makewidgets(self, options):
        sbar = Scrollbar(self)
        list = Listbox(self, relief=SUNKEN)

        sbar.pack(side=RIGHT, fill=Y)
        list.pack(side=LEFT, expand=YES, fill=BOTH)

        pos = 0
        for label in options:
            list.insert(pos, label)
            pos += 1

        sbar.config(command=list.yview)
        list.config(yscrollcommand=sbar.set)

        list.bind('<Double-1>', self.handleList)
        # list.bind('<Button-1>', self.handleList)
        self.listbox = list

    def handleList(self, event):
        # index = self.listbox.curselection()
        # label = self.listbox.get(index)
        label = self.listbox.get('active')
        self.runCommand(label)

    def runCommand(self, selection):
        print('You selected:', selection)

if __name__ == '__main__':
    options = ['zhiwen he - %s' % i for i in range(20)]
    ScrolledList(options).mainloop()
