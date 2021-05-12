def once(f):
    called = False
    def new_f(*args):
        nonlocal called
        if not called:
            called = True
            f(*args)
    return new_f

@once
def func1(arg1, arg2):
    print('func1', arg1, arg2)

@once
def func2(arg1, arg2):
    print('func2', arg1, arg2)

print('call func1 two times')
func1(1, 2)
func1(1, 2)

print('call func2 two times')
func2(1, 2)
func2(1, 2)

def func3(arg1, arg2):
    print('func3', arg1, arg2)

print('call func3 two times')
func3(1, 2)
func3(1, 2)

func3 = once(func3)

print('call func3 two times')
func3(1, 2)
func3(1, 2)
