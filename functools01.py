from functools import partial

def func(a, b, c):
    print(a, b, c)

def func1(a, b, c):
    print(a, b, c)

if __name__ == '__main__':
    print(func.__name__)
    func('a', 'b', 'c')
    func('a', c = 'c', b = 'b')

    pfunc1 = partial(func1, 'a', 'b')
    pfunc1()
