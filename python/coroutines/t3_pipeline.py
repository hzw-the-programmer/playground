def grep(pattern, lines):
  for line in lines:
    if pattern in line:
      yield line

if __name__ == '__main__':
  from t2_follow import follow

  logfile = open('access-log')
  loglines = follow(logfile)
  pylines = grep('python', loglines)

  for line in pylines:
    print(line)
