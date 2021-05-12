from base64 import b64decode
from binascii import hexlify
import sys

if len(sys.argv) != 3:
    print('Usage: {} d|e str'.format(sys.argv[0]))

if sys.argv[1] == 'd':
    print(hexlify(b64decode(sys.argv[2])))
