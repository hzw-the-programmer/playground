def counter():
    count = 0
    def _counter():
        nonlocal count
        count += 1
        return count
    return _counter

c = counter()
print(c())
print(c())

def gen_counter():
    count = 0
    while True:
        count += 1
        yield count

c = gen_counter()
print(next(c))
print(next(c))
