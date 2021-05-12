from binascii import hexlify
from binascii import unhexlify
import glob
import fileinput
import os
import sys

from utils import unpack_header
from utils import unpack_id

if len(sys.argv) < 2:
    print('usage: {0} dir'.format(sys.argv[0]))
    sys.exit()

for file in glob.glob(sys.argv[1] + '/**/*.rewrite', recursive=True):
    os.remove(file)

for line in fileinput.input(glob.glob(sys.argv[1] + '/**/*.txt', recursive=True)):
    (date, time, rw, ip, port, data) = line.split()
    (valid, protocal, length, sn, id, pkt_time, cmd, data) = unpack_header(unhexlify(data))
    if cmd == 0x24:
        id = unpack_id(data)[0]

    with open(fileinput.filename() + '.rewrite', 'a') as f:
        f.write('{rw} 0x{cmd:x} 0x{id:08x} {pkt_time} {data}\n'.format(valid=valid, protocal=protocal, length=length, sn=sn, id=id, pkt_time=pkt_time, cmd=cmd, data=hexlify(data), rw=rw))
