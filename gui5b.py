from gui5 import HelloButton

class MyBotton(HelloButton):
    def callback(self):
        print('ignoring press!...')

if __name__ == '__main__':
    MyBotton(None, text='Hello subclass world').mainloop()
