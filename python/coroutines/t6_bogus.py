def countdown(n):
  print('Counting down from', n)

  while n >= 0:
    newValue = yield n

    if newValue is not None:
      n = newValue
    else:
      n -= 1

if __name__ == '__main__':
  c = countdown(5)

  for i in c:
    print(i)
    if i == 5:
      c.send(3)
      