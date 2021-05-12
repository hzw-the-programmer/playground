def countdown(n):
  print('Counting down from', n)

  while (n > 0):
    yield n
    n -= 1

  print('Done counting down')

if __name__ == '__main__':
  # cd = countdown(3)
  # print(cd)
  # print(next(cd))
  # print(next(cd))
  # print(next(cd))
  # next(cd)

  for i in countdown(10):
    print(i)
