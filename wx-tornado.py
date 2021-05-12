import hashlib
import json
import tornado.ioloop
import tornado.web
from config import config
try:
  from urllib.request import request
except:
  import urllib
  request = urllib
import uuid

import receive
import reply
import wx_access_token

class MainHandler(tornado.web.RequestHandler):
  def get(self):
    try:
      signature = self.get_argument('signature')
      timestamp = self.get_argument('timestamp')
      nonce = self.get_argument('nonce')
      echostr = self.get_argument('echostr')
      print(signature, timestamp, nonce, echostr)

      list = [config['token'], timestamp, nonce]
      list.sort()
      sha1 = hashlib.sha1()
      map(sha1.update, list)
      hashcode = sha1.hexdigest()
      print('hashcode, signature', hashcode, signature)
      if hashcode == signature:
        self.write(echostr)
      else:
        self.write('')
    except Exception, Argument:
      print Argument
      self.write('exception')

  def post(self):
    try:
      data = self.request.body
      print data
      recMsg = receive.parse_xml(data)
      if isinstance(recMsg, receive.Msg):
        toUser = recMsg.FromUserName
        fromUser = recMsg.ToUserName
        replyMsg = reply.Msg()
        if recMsg.MsgType == 'text':
          replyMsg = reply.TextMsg(toUser, fromUser, recMsg.Content)
        elif recMsg.MsgType == 'image':
          replyMsg = reply.ImageMsg(toUser, fromUser, recMsg.MediaId)
        self.write(replyMsg.send())
      else:
        print('ignore')
        self.write('success')
    except Exception as e:
      print(e)
      self.write(e)

class WxkHandler(tornado.web.RequestHandler):
  def post(self):
    body = self.request.body
    print(body)
    body = json.loads(body)
    msg = '{"filter": {"is_to_all": true}, "text": {"content": "%s"}, "msgtype": "text", "clientmsgid": "%s"}' \
          % (body['text'], uuid.uuid4().hex)
    resp = wx_access_token.get()
    url = 'https://api.weixin.qq.com/cgi-bin/message/mass/sendall?access_token={access_token}'.format(**resp)
    resp = request.urlopen(url, msg.encode('utf-8'))
    print(resp.read())

def make_app():
  return tornado.web.Application([
    (r'/wx', MainHandler),
    (r'/wxk', WxkHandler)
  ])

if __name__ == '__main__':
  app = make_app()
  app.listen(80)
  tornado.ioloop.IOLoop().current().start()
