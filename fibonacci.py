'''Fibonacci generator (stage 1)

Command line:
$ python3 fibonacci 10
0 1 1 2 3 5 8

'''

def fib(max):
    a, b = 0, 1
    while a < max:
        yield a
        a, b = b, a + b

if __name__ == '__main__':
    import sys
    if sys.argv[1:]:
        fib_gen = fib(int(sys.argv[1]))
        for i in fib_gen:
            print(i, end = ' ')
        print()
        for i in fib_gen:
            print(i, end = ' ')
        print()
    else:
        print(__doc__)
