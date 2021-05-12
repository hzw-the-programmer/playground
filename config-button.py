from tkinter import *

widget = Button(text='Spam', padx=10, pady=10)
# widget = Button(text='Spam', padx=40, pady=40)
# widget.pack(padx=20, pady=20)
widget.pack(padx=40, pady=40)
widget.config(cursor='gumby')
widget.config(bd=8, relief=RAISED)
# widget.config(bd=16, relief=RAISED)
widget.config(font=('helvetica', 20, 'underline italic'))
mainloop()
