import json
try:
  from urllib.request import request
except ImportError:
  import urllib
  request = urllib
from config import config

def get():
    url = 'https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={appid}&secret={appsecret}'.format(**config)
    resp = request.urlopen(url)
    resp = json.loads(resp.read())
    return resp

if __name__ == '__main__':
    print(get())
