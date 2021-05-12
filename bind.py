from tkinter import *

def showPosEvent(event):
    print('Widget=%s X=%s Y=%s' % (event.widget, event.x, event.y))

def showAllEvent(event):
    print(event)
    for attr in dir(event):
        if not attr.startswith('__'):
            print(attr, '=>', getattr(event, attr))

def onLeftClick(event):
    print('Got left mouse button click:', end=' ')
    showPosEvent(event)

def onRightClick(event):
    print('Got right mouse button click:', end=' ')
    showPosEvent(event)

def onMiddleClick(event):
    print('Got middle mouse button click:', end=' ')
    showPosEvent(event)
    showAllEvent(event)

def onDoubleLeftClick(event):
    print('Got left mouse button double click:', end=' ')
    showPosEvent(event)

def onDoubleRightClick(event):
    print('Got right mouse button double click:', end=' ')
    showPosEvent(event)

def onDoubleMiddleClick(event):
    print('Got middle mouse button double click:', end=' ')
    showPosEvent(event)

def onLeftDrag(event):
    print('Got left mouse button drag:', end=' ')
    showPosEvent(event)

def onRightDrag(event):
    print('Got right mouse button drag:', end=' ')
    showPosEvent(event)

def onMiddleDrag(event):
    print('Got middle mouse button drag:', end=' ')
    showPosEvent(event)

def onKeyPress(event):
    print('Got key press:', event.keycode, event.char)
    showAllEvent(event)

def onUpKeyPress(event):
    print('Got up key press:', event.keycode, event.char)
    showAllEvent(event)

def onReturnKeyPress(event):
    print('Got return key press:', event.keycode, event.char)
    showAllEvent(event)

root = Tk()
widget = Button(root, text='Hello bind world')
widget.config(bg='red', font=('courier', 20, 'bold'))
widget.config(height=5, width=20)
widget.pack(expand=YES, fill=BOTH)

widget.bind('<Button-1>', onLeftClick)
widget.bind('<Button-3>', onRightClick)
widget.bind('<Button-2>', onMiddleClick)

widget.bind('<Double-1>', onDoubleLeftClick)
widget.bind('<Double-3>', onDoubleRightClick)
widget.bind('<Double-2>', onDoubleMiddleClick)

widget.bind('<B1-Motion>', onLeftDrag)
widget.bind('<B3-Motion>', onRightDrag)
widget.bind('<B2-Motion>', onMiddleDrag)

widget.bind('<KeyPress>', onKeyPress)
widget.bind('<Up>', onUpKeyPress)
widget.bind('<Return>', onReturnKeyPress)

widget.focus()
root.title('Click Me')
root.mainloop()
