def gen():
    try:
        while True:
            print('before yield')
            yield
            print('after yield')
    except GeneratorExit as ge:
        print('close called')

def gen1():
    while True:
        print('before yield')
        yield
        print('after yield')

if __name__ == '__main__':
    # g = gen()
    # g.__next__()
    # g.__next__()
    # g.close()
    # g.__next__()

    g = gen1()
    g.__next__()
    g.__next__()
    g.close()
    g.__next__()
