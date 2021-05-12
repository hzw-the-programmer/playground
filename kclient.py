import datetime
import socket
import sys
from binascii import hexlify

from utils import pack_sn, pack_id, pack_datetime, pack_51, pack_header, \
                  pack_41

if len(sys.argv) != 3:
    print("Usage: " + sys.argv[0] + " sn ip")
    exit()

sn = sys.argv[1]
ip = sys.argv[2]

protocal = 0x33
id = 0x12345678
dt = datetime.datetime.now()

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

ddatas = [
    (1, 0, 8, 0x00),
    (1, 1, 9, 0x20),
    (1, 2, 10, 0x00),
    (1, 3, 11, 0x20),
    (1, 4, 12, 0x00),
    (1, 5, 13, 0x20),
]

data = bytearray()
data.extend(pack_sn(sn))
data.extend(pack_id(id))
data.extend(pack_datetime(dt))

data.extend(pack_51(ddatas))
# data.extend(pack_41([(1, 0, 12, 20)]))

data = pack_header(protocal, data)
print(hexlify(data))
sock.sendto(data, (ip, 19268))
