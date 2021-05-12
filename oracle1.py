import cx_Oracle

connection = cx_Oracle.connect('kaifa/kaifa@orcl')
cursor = connection.cursor()
cursor.execute('SELECT COUNT(*) FROM channel_ad')
count = cursor.fetchall()[0][0]
print(count)
connection.close()
