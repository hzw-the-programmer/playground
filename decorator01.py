def deco(f):
    def d():
        print('deco exe')
        f()
    return d

@deco
def func():
    '''func doc
    '''
    print('func exe')

if __name__ == '__main__':
    func()
    print(func.__name__)
    print(func.__doc__)
