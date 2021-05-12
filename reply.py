import time

class Msg(object):
    def __init__(self, toUserName=None, fromUserName=None, msgType=None):
        self.dict = {}
        self.dict['ToUserName'] = toUserName
        self.dict['FromUserName'] = fromUserName
        self.dict['CreateTime'] = int(time.time())
        self.dict['MsgType'] = msgType

    def send(self):
        if self.dict['ToUserName'] == None:
          return 'success'
        result = '<xml>\n'
        result += '\t<ToUserName><![CDATA[{ToUserName}]]></ToUserName>\n'
        result += '\t<FromUserName><![CDATA[{FromUserName}]]></FromUserName>\n'
        result += '\t<CreateTime>{CreateTime}</CreateTime>\n'
        result += '\t<MsgType><![CDATA[{MsgType}]]></MsgType>\n'
        result += self.send_t()
        result += '</xml>'
        return result.format(**self.dict)

    def send_t(self):
        pass

class TextMsg(Msg):
    def __init__(self, toUserName, fromUserName, content):
        Msg.__init__(self, toUserName, fromUserName, 'text')
        self.dict['Content'] = content

    def send_t(self):
        return '\t<Content><![CDATA[{Content}]]></Content>\n'

class ImageMsg(Msg):
    def __init__(self, toUserName, fromUserName, mediaId):
        Msg.__init__(self, toUserName, fromUserName, 'image')
        self.dict['MediaId'] = mediaId

    def send_t(self):
        result = '\t<Image>\n'
        result += '\t\t<MediaId><![CDATA[{MediaId}]]></MediaId>\n'
        result += '\t</Image>\n'
        return result

if __name__ == '__main__':
    print(TextMsg('ZhiWen He', 'wx_gzpt_hzw_01@qq.com', 'hello, ZhiWen He love you!').send())
    print(ImageMsg('ZhiWen He', 'wx_gzpt_hzw_01@qq.com', 123456).send())
