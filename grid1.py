from tkinter import *

colors = ['red', 'green', 'orange', 'white', 'yellow', 'blue']

if __name__ == '__main__':
    row = 0
    for c in colors:
        Label(text=c, relief=RIDGE, width=25).grid(row=row, column=0)
        Entry(bg=c, relief=SUNKEN, width=50).grid(row=row, column=1)
        row += 1
    mainloop()
