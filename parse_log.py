from binascii import unhexlify
import fileinput

from utils import unpack_header
from utils import unpack_id

if __name__ == '__main__':
    stat = {'W': {'total': 0, 'ack': 0, 'data': 0, 'ids': {}},
            'R': {'total': 0, 'ack': 0, 'data': 0, 'ids': {}}}

    for line in fileinput.input():
        (date, time, rw, ip, port, data) = line.split()
        (valid, protocal, length, sn, id, pkt_time, cmd, data) = unpack_header(unhexlify(data))

        stat[rw]['total'] += 1
        if cmd == 0x24:
            stat[rw]['ack'] += 1
            ack_id = unpack_id(data)[0]
            if rw == 'W':
                if ack_id in stat['R']['ids']:
                    stat['R']['ids'][ack_id]['acked'] += 1
                else:
                    raise Exception('line: %d' % fileinput.lineno())
            else:
                if ack_id in stat['W']['ids']:
                    stat['W']['ids'][ack_id]['acked'] += 1
                else:
                    raise
        else:
            stat[rw]['data'] += 1
            if id in stat[rw]['ids']:
                stat[rw]['ids'][id]['total'] += 1
            else:
                stat[rw]['ids'][id] = {'total': 1, 'acked': 0}

    for k, v in stat.items():
        print(k)
        print('total: %s' % (v['total']))
        print('ack: %s' % (v['ack']))
        print('data: %s' % (v['data']))

    for k, v in stat.items():
        print(k)
        for k1, v1 in v['ids'].items():
            if v1['total'] != v1['acked']:
                print('%s: total: %s acked: %s' % (k1, v1['total'], v1['acked']))
