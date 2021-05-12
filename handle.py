import hashlib
import web
from config import config

class Handle(object):
  def GET(self):
    try:
      data = web.input()
      if len(data) == 0:
        return 'hello, this is handle view'
      signature = data.signature
      timestamp = data.timestamp
      nonce = data.nonce
      echostr = data.echostr
      token = config['token']
      list = [token, timestamp, nonce]
      list.sort()
      sha1 = hashlib.sha1()
      map(sha1.update, list)
      hashcode = sha1.hexdigest()
      print 'handle/GET func: hashcode, signature: ', hashcode, signature
      if hashcode == signature:
        return echostr
      else:
        return ''
    except Exception, Argument:
      return Argument
