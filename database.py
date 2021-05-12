import pymysql
import queue
import threading

class database:
    HOST = 'localhost'
    USER = 'hzw'
    PASSWORD = '123456'
    DATABASE = 'iot0'

    con = None
    que = None
    thread = None

    @classmethod
    def initdb(cls):
        con = None
        try:
            con = pymysql.connect(host=cls.HOST,
                                  user=cls.USER,
                                  password=cls.PASSWORD)
            with con.cursor() as cursor:
                # sql = 'DROP DATABASE IF EXISTS {0}'.format(cls.DATABASE)
                # cursor.execute(sql)

                sql = 'CREATE DATABASE IF NOT EXISTS {0}'.format(cls.DATABASE)
                cursor.execute(sql)

                sql = 'CREATE TABLE IF NOT EXISTS {0}.device ( \
                    id INT NOT NULL AUTO_INCREMENT, \
                    sn VARCHAR(50) NOT NULL, \
                    ip VARCHAR(50) NOT NULL, \
                    port INT NOT NULL, \
                    PRIMARY KEY (id) \
                )'.format(cls.DATABASE)
                cursor.execute(sql)

                sql = 'CREATE TABLE IF NOT EXISTS {0}.channel ( \
                    id INT NOT NULL AUTO_INCREMENT, \
                    did INT NOT NULL, \
                    slot INT NOT NULL, \
                    port INT NOT NULL, \
                    type INT NOT NULL, \
                    PRIMARY KEY (id), \
                    FOREIGN KEY fk_did (did) REFERENCES {0}.device (id) \
                )'.format(cls.DATABASE)
                cursor.execute(sql)

                sql = 'CREATE TABLE IF NOT EXISTS {0}.status ( \
                    id INT NOT NULL AUTO_INCREMENT, \
                    cid INT NOT NULL, \
                    ctime datetime NOT NULL, \
                    data INT NOT NULL, \
                    PRIMARY KEY (id), \
                    FOREIGN KEY fk_cid (cid) REFERENCES {0}.channel (id) \
                )'.format(cls.DATABASE)
                cursor.execute(sql)

                sql = 'CREATE TABLE IF NOT EXISTS {0}.places ( \
                    id INT NOT NULL AUTO_INCREMENT, \
                    name NVARCHAR(50) NOT NULL, \
                    pid INT NOT NULL, \
                    PRIMARY KEY (id) \
                )'.format(cls.DATABASE)
                cursor.execute(sql)

        except Exception as e:
            raise
        else:
            pass
        finally:
            if con is not None:
                con.close()

    @classmethod
    def getdbcon(cls):
        if cls.con is None:
            cls.con = pymysql.connect(host=cls.HOST,
                                      user=cls.USER,
                                      password=cls.PASSWORD,
                                      db=cls.DATABASE)
        return cls.con

    @classmethod
    def initque(cls):
        cls.thread = threading.Thread(target=cls.processque)
        cls.thread.daemon = True
        cls.thread.start()

    @classmethod
    def processque(cls):
        while True:
            action, data = cls.deque()
            if action == 0x51:
                sn, addr, datetime, data = data
                try:
                    with cls.getdbcon().cursor() as cursor:
                        select = 'SELECT id, sn, ip, port \
                                  FROM device \
                                  WHERE sn = %s'
                        cursor.execute(select, (sn,))
                        row = cursor.fetchone()
                        if row is None:
                            insert = 'INSERT INTO device \
                                      (sn, ip, port) \
                                      VALUES \
                                      (%s, %s, %s)'
                            cursor.execute(insert, (sn, addr[0], addr[1]))
                            cls.getdbcon().commit()
                            cursor.execute(select, (sn,))
                            row = cursor.fetchone()

                        did, sn, ip, port = row
                        if ip != addr[0] or port != addr[1]:
                            ip = addr[0]
                            port = addr[1]
                            update = 'UPDATE device \
                                      SET ip = %s, port = %s \
                                      WHERE sn = %s'
                            cursor.execute(update, (ip, port, sn))
                            cls.getdbcon().commit()

                        for slot, port, type, ddata in data:
                            select = 'SELECT id, did, slot, port, type \
                                      FROM channel \
                                      WHERE did = %s AND slot = %s AND port = %s'
                            cursor.execute(select, (did, slot, port))
                            row = cursor.fetchone()
                            if row is None:
                                insert = 'INSERT INTO channel \
                                          (did, slot, port, type) \
                                          VALUES \
                                          (%s, %s, %s, %s)'
                                cursor.execute(insert, (did, slot, port, type))
                                cls.getdbcon().commit()
                                cursor.execute(select, (did, slot, port))
                                row = cursor.fetchone()

                            cid, did, slot, port, type = row
                            select = 'SELECT cid, ctime \
                                      FROM status \
                                      WHERE cid = %s AND ctime = %s'
                            cursor.execute(select, (cid, datetime))
                            row = cursor.fetchone()
                            if row is None:
                                insert = 'INSERT INTO status \
                                          (cid, ctime, data) \
                                          VALUES \
                                          (%s, %s, %s)'
                                cursor.execute(insert, (cid, datetime, ddata))
                                cls.getdbcon().commit()

                except Exception as e:
                    raise

    @classmethod
    def getque(cls):
        if cls.que is None:
            cls.que = queue.Queue()
        return cls.que

    @classmethod
    def enque(cls, data):
        cls.getque().put(data)

    @classmethod
    def deque(cls):
        return cls.getque().get()

    @classmethod
    def init(cls):
        cls.initdb()
        cls.initque()

if __name__ == '__main__':
    database.init()
