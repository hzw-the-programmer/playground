def entryExit(f):
  def new_f():
    print('Entering', f.__name__)
    f()
    print('Exited', f.__name__)

  # new_f.__name__ = f.__name__
  return new_f

def func1():
  print('inside func1')
func1 = entryExit(func1)

@entryExit
def func2():
  print('inside func2')

func1()
func2()
print(func2.__name__)
