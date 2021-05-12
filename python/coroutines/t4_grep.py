def grep(pattern):
  print('Looking for %s' % pattern)

  while True:
    line = yield
    if pattern in line:
      print(line)

if __name__ == '__main__':
  g = grep('python')
  next(g)
  g.send('Yeah, but no, but yeah, but no')
  g.send('A series of tubes')
  g.send('python generator rock!')
  