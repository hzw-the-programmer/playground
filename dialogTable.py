from tkinter.filedialog import askopenfilename, asksaveasfilename, askdirectory
from tkinter.colorchooser import askcolor
from tkinter.messagebox import askquestion, showerror, askyesno, askokcancel
from tkinter.simpledialog import askinteger, askfloat, askstring

demos = {
    'Open': askopenfilename,
    'Save As': asksaveasfilename,
    'Directory': askdirectory,
    'Color': askcolor,
    'Query': lambda: askquestion('Warning', 'You typed "rm *"\nConfirm?'),
    'Error': lambda: showerror('Error', "He's dead, Jim"),
    'Yes/No': lambda: askyesno('Yes/No', 'haha'),
    'Ok/Cancel': lambda: askokcancel('Ok/Cancel', 'hehe'),
    'Integer': lambda: askinteger('Integer', 'Enter Ineteger'),
    'Float': lambda: askfloat('Float', 'Enter Float'),
    'String': lambda: askstring('String', 'Enter String'),
}
