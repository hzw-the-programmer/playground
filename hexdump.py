import sys
from binascii import hexlify

if len(sys.argv) < 2:
    print('Usage: python %s filename' % sys.argv[0])
    sys.exit(2);
filename = sys.argv[1]

with open(filename, 'rb') as file:
    print(hexlify(file.read()))
