from tkinter import *

root = Tk()
widget = Label(root)
widget.config(text='Hello GUI world!', bg='red')
# widget.pack(side=TOP, expand=YES)
# widget.pack(side=TOP, fill=BOTH)
widget.pack(side=TOP, expand=YES, fill=BOTH)
root.title('gui1g.py')
root.mainloop()
