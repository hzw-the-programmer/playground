from binascii import unhexlify
from collections import UserDict
from sys import modules
from utils import unpack_header



class Pkt(UserDict):
    def __init__(self, valid, protocal, length, name, id, time, cmd, data):
        UserDict.__init__(self)
        self['valid'] = valid
        self['protocal'] = protocal
        self['length'] = length
        self['name'] = name
        self['id'] = id
        self['time'] = time
        self['cmd'] = cmd
        self.unpack(data)

    @staticmethod
    def create(data):
        (valid, protocal, length, name, id, time, cmd, data) = unpack_header(data)
        subclass = 'Pkt{0:x}'.format(cmd)
        return getattr(modules[Pkt.__module__], subclass)(valid, protocal, length, name, id, time, cmd, data)

    def unpack(self, data):
        pass

class Pkt01(Pkt):
    def unpack(self, data):
        self[]

class Pkt02(Pkt):
    def unpack(self, data):
        self[]

if __name__ == '__main__':
    pkt = Pkt.create(unhexlify('5033001DF3C5015E13E50003000000011105140905304301F3C5015E13E500037F'))
    print(pkt)
