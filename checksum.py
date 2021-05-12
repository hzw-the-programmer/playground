from binascii import unhexlify
import sys
from utils import checksum

print('{0:x}'.format(checksum(unhexlify(sys.argv[1]))))
