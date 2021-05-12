import gui7

class HelloPackage(gui7.HelloPackage):
    # pass
    def __getattr__(self, name):
        return getattr(self.top, name)

if __name__ == '__main__':
    HelloPackage().mainloop()
