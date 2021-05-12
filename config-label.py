from tkinter import *

root = Tk()
widget = Label(root, text='Hello config world')
widget.config(bg='black', fg='yellow')
labelfont = ('times', 20, 'bold')
widget.config(font=labelfont)
widget.config(height=3, width=20)
# widget.pack(expand=YES)
# widget.pack(fill=X)
# widget.pack(fill=Y)
widget.pack(expand=YES, fill=Y)
root.mainloop()
