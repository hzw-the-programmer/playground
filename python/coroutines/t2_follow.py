import time

def follow(thefile):
  thefile.seek(0, 2)
  while True:
    line = thefile.readline()

    if not line:
      print('line is empty string')
      time.sleep(1)
      # time.sleep(5)
      # time.sleep(0.1)
      continue

    yield line

if __name__ == '__main__':
  logfile = open('access-log')
  for line in follow(logfile):
    print(line)