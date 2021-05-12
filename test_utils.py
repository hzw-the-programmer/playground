from binascii import hexlify
from binascii import unhexlify
import unittest
from datetime import datetime
from utils import pack_sn, unpack_sn, \
                  pack_id, unpack_id, \
                  pack_datetime, unpack_datetime, \
                  pack_41

class UtilsTestCase(unittest.TestCase):
    def test_pack_sn(self):
        self.assertEqual(b'32f661fe01480000',
                         hexlify(pack_sn('79173400000050')))

    def test_unpack_sn(self):
        self.assertEqual('79173400000050',
                         unpack_sn(unhexlify(b'32f661fe01480000'))[0])

    def test_pack_id(self):
        self.assertEqual(b'12345678', hexlify(pack_id(0x12345678)))

    def test_unpack_id(self):
        self.assertEqual(0x12345678, unpack_id(unhexlify(b'12345678'))[0])

    def test_pack_datetime(self):
        dt = datetime(2017, 1, 23, 4, 56, 7)
        self.assertEqual(b'110117043807', hexlify(pack_datetime(dt)))

    def test_unpack_datetime(self):
        dt = datetime(2017, 1, 23, 4, 56, 7)
        self.assertEqual(dt, unpack_datetime(unhexlify(b'110117043807'))[0])

    def test_pack_41(self):
        self.assertEqual(b'410101010ca4709d3f', hexlify(pack_41([(1, 1, 12, 1.23)])))

if __name__ == '__main__':
    unittest.main()
