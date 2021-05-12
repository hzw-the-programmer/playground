import sys

fin = sys.argv[1]

fi = open(fin, 'r')
fo = open(fin + '.out', 'w')

data = fi.read()

i = 0
length = len(data)

while i < length:
    if i < length - 4:
        seq = data[i:i + 5]
        if seq == '24 46' or seq == '24 41':
            fo.write('\n')
            fo.write(seq)
            i += 5
            if seq == '24 46':
                i += 33
            else:
                i += 30

    fo.write(data[i])
    i += 1

fi.close()
fo.close()
