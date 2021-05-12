import json
try:
  from urllib.request import request
except ImportError:
  import urllib
  request = urllib
import wx_access_token

def send():
    msg = '{"filter": {"is_to_all": true}, "text": {"content": "I am ZhiWenHe"}, "msgtype": "text"}'
    print(msg)
    resp = wx_access_token.get()
    url = 'https://api.weixin.qq.com/cgi-bin/message/mass/sendall?access_token={access_token}'.format(**resp)
    resp = request.urlopen(url, msg.encode('utf-8'))
    print(resp.read())

if __name__ == '__main__':
    send()
