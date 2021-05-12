def coroutine(func):
  def start(*args, **kwargs):
    cr = func(*args, **kwargs)
    next(cr)
    return cr

  return start

if __name__ == '__main__':
  @coroutine
  def grep(pattern):
    print('Looking for %s' % pattern)

    while True:
      line = yield

      if pattern in line:
        print(line, end='')

  g = grep('python')
  g.send('Yeah, but no, but yeah, but no')
  g.send('A series of tubes')
  g.send('python generators rock!')
