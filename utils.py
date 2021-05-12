from binascii import hexlify
from binascii import unhexlify
from datetime import datetime
import fileinput
from struct import pack;
from struct import unpack;

def type_to_str(type):
    type_strs = { 8: 'GND-H',
                  9: 'WS',
                 10: 'VB',
                 12: 'GND-L'}
    return type_strs[type]

def unpack_sn(data):
    (sn,) = unpack('<q', data[:8])
    data = data[8:]

    return (str(sn), data)

def pack_sn(sn):
    return pack('<q', int(sn))

def unpack_id(data):
    id = unpack('>I', data[:4])[0]
    data = data[4:]

    return (id, data)

def pack_id(id):
    return pack('>I', id)

def unpack_datetime(data):
    (year, month, day, hour, minute, second) = unpack('6B', data[:6])
    data = data[6:]

    try:
        dt = datetime(year + 2000, month, day, hour, minute, second)
    except:
        dt = datetime(1987, 1, 23)

    return (dt, data)

def pack_datetime(datetime):
    year = datetime.year - 2000
    month = datetime.month
    day = datetime.day
    hour = datetime.hour
    minute = datetime.minute
    second = datetime.second
    # print(year, month, day, hour, minute, second)
    return pack('6B', year, month, day, hour, minute, second)

def checksum(data):
    result = 0
    for d in data:
        result ^= d
    return result

def unpack_header(data):
    valid = False
    protocal = 0x32
    length = 0
    sn = ''
    id = 0
    datetime = 0
    cmd = 0

    mark = data[0]
    data = data[1:]

    if mark == 0x50:
        protocal = data[0]
        data = data[1:]

        if protocal == 0x33:
            length = unpack('>H', data[:2])[0]
            data = data[2:]
        else:
            length = data[0]
            data = data[1:]

        if length == len(data):
            chksum = data[-1]
            data = data[:-1]

            if chksum == checksum(data):
                valid = True

                if protocal == 0x33:
                    (sn, data) = unpack_sn(data)
                    (id, data) = unpack_id(data)
                    (datetime, data) = unpack_datetime(data)

                cmd = data[0]
                data = data[1:]

    return (valid, protocal, length, sn, id, datetime, cmd, data)

def pack_header(protocal, data):
    result = bytearray()
    result.append(0x50)
    result.append(protocal)

    data.append(checksum(data))
    length = len(data)
    if (protocal == 0x33):
        result.extend(pack('>H', length))
    else:
        result.append(length)

    result.extend(data)
    return result

def pack_51(data):
    result = bytearray()
    result.append(0x51)
    result.append(len(data))
    for d in data:
        result.append((d[0] << 4) | (d[1] & 0x0F))
        result.append(d[2] & 0x0F)
        result.append(d[3])
    return result

def pack_51_req(sn, data):
    return pack_51(sn, data)

def unpack_51(data):
    (sn, data) = unpack_sn(data)
    length = data[0]
    data = data[1:]

    result = []
    for i in range(length):
        slot = data[0] >> 4
        port = data[0] & 0x0F
        type = data[1] & 0x0F
        ddata = data[2]
        data = data[3:]
        result.append((slot, port, type, ddata))
    return (sn, result, data)

def unpack_51_req(data):
    return unpack_51(data)

def pack_41(data):
    result = bytearray()
    result.append(0x41)
    result.append(len(data))
    for d in data:
        result.append(d[0])
        result.append(d[1])
        result.append(d[2])
        result.extend(pack('<f', d[3]))
    return result

if __name__ == '__main__':
    sn = '20170123000001'
    if unpack_sn(pack_sn(sn))[0] != sn:
        raise

    id = 0x12345678
    if unpack_id(pack_id(id))[0] != id:
        raise

    dt = datetime(2017, 1, 23, 11, 11, 11)
    if unpack_datetime(pack_datetime(dt))[0] != dt:
        raise

    origin_data = unhexlify('5033003BF3D9015E13E50003000001C611061200003751F3D9015E13E500030A100C08110C08120C08130C08200C08210C08220C08230C08400918410978CF')
    # unpack
    (valid, protocal, length, sn, id, dt, cmd, data) = unpack_header(origin_data)
    (sn, ddatas, data) = unpack_51_req(data)

    # pack
    data = bytearray()
    data.extend(pack_sn(sn))
    data.extend(pack_id(id))
    data.extend(pack_datetime(dt))
    data.extend(pack_51_req(sn, ddatas))
    if pack_header(protocal, data) != origin_data:
        raise
