'''Fibonacci iterator (stage 2)

Command line:
$ python3 fibonacci2 10
0 1 1 2 3 5 8
0 1 1 2 3 5 8 
'''

class Fib:
    '''iterator that yields numbers in the Fibonacci sequence'''

    def __init__(self, max):
        self.max = max

    def __iter__(self):
        self.a = 0
        self.b = 1
        return self

    def __next__(self):
        fib = self.a
        if fib > self.max:
            raise StopIteration
        self.a, self.b = self.b, self.a + self.b
        return fib

if __name__ == '__main__':
    import sys
    if sys.argv[1:]:
        fib_inst = Fib(int(sys.argv[1]))
        for i in fib_inst:
            print(i, end = ' ')
        print()
        for i in fib_inst:
            print(i, end = ' ')
        print()
    else:
        print(__doc__)
