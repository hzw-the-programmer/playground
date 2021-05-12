class entryExit(object):
  def __init__(self, f):
    print('Inside Decoration.')
    self.f = f

  def __call__(self):
    print('Entering', self.f.__name__)
    self.f()
    print('Exited', self.f.__name__)

print('Before decoration.')
@entryExit
def func1():
  print('inside func1')
print('After decoration.')
print(func1)

# print(func1.__name__)
func1()
