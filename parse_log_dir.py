from binascii import unhexlify
import cx_Oracle
import fileinput
from glob import glob
import sys

from utils import unpack_header
from utils import unpack_sn
from utils import type_to_str

if __name__ == '__main__':
    conn = cx_Oracle.connect('kaifa/kaifa@orcl')
    count = 0

    for line in fileinput.input(files=glob('log/**/*.txt', recursive=True)):
        (date, time, rw, ip, port, data) = line.split()
        (valid, protocal, length, sn, id, pkt_time, cmd, data) = unpack_header(unhexlify(data))

        if rw == 'R' and cmd == 0x51:
            (sn, data) = unpack_sn(data)

            l = data[0];
            data = data[1:]

            i = 0
            while i < l:
                did = -1
                pid = -1

                slot = data[0] >> 4
                port = data[0] & 0x0F
                type = type_to_str(data[1])
                ddata = data[2]
                data = data[3:]

                # cursor = conn.cursor()
                # cursor.execute("""
                #      SELECT did
                #      FROM device
                #      WHERE sn=:sn AND slot=:slot AND port=:port AND devicetype=:type""",
                #      sn=sn, slot=slot, port=port, type=type)
                # r = cursor.fetchall()
                # if len(r) != 0:
                #     did = r[0][0]
                # else:
                #     print(ip, pkt_time, sn, slot, port, 'no did', file=sys.stderr)
                # cursor.close()
                #
                # if did != -1:
                #     cursor = conn.cursor()
                #     cursor.execute("""
                #          SELECT plant, workshop, region, line, station, channel
                #          FROM graph_station
                #          WHERE sn=:sn AND slot=:slot AND port=:port AND devicetype=:type""",
                #          sn=sn, slot=slot, port=port, type=type)
                #     r = cursor.fetchall()
                #     (plant, workshop, region, line, station, channel) = (None, None, None, None, None, None)
                #     if len(r) != 0:
                #         (plant, workshop, region, line, station, channel) = r[0]
                #     else:
                #         print(ip, pkt_time, sn, slot, port, 'no plant', file=sys.stderr)
                #     cursor.close()
                #
                #     if plant != None:
                #         cursor = conn.cursor()
                #         cursor.execute("""
                #              SELECT pid
                #              FROM place
                #              WHERE plant=:plant AND workshop=:workshop AND region=:region AND line=:line AND station=:station AND channel=:channel""",
                #              plant=plant, workshop=workshop, region=region, line=line, station=station, channel=channel)
                #         r = cursor.fetchall()
                #         if len(r) != 0:
                #             pid = r[0][0]
                #         else:
                #             print(ip, pkt_time, sn, slot, port, 'no pid', file=sys.stderr)
                #         cursor.close()

                cursor = conn.cursor()
                cursor.execute("""
                       SELECT b.did, c.pid FROM graph_station a, device b, place c
                       WHERE a.sn = b.sn AND a.slot = b.slot AND a.port = b.port AND a.devicetype = b.devicetype AND
                             a.plant = c.plant AND a.workshop = c.workshop AND a.region = c.region AND a.line = c.line AND a.station = c.station AND a.channel = c.channel AND
                             a.sn = :sn AND a.slot = :slot AND a.port = :port AND a.devicetype = :type""",
                             sn=sn, slot=slot, port=port, type=type)
                r = cursor.fetchall()
                if len(r) != 0:
                    (did, pid) = r[0]
                cursor.close()

                if did != -1 and pid != -1:
                    cursor = conn.cursor()
                    cursor.execute("""
                           INSERT INTO ddata
                           VALUES(ddata_seq.nextval, :did, :pid, :ddata, TO_DATE(:time, 'YYYY MM DD HH24 MI SS'))""",
                           did=did, pid=pid, ddata=ddata, time=pkt_time.strftime('%Y %m %d %H %M %S'))
                    cursor.close()
                    conn.commit()
                    count += 1

                print(did, pid)
                i += 1

    print(count)
    conn.close()
