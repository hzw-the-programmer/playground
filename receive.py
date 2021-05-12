import xml.etree.ElementTree as ET

def parse_xml(data):
    if len(data) == 0:
        return None
    data = ET.fromstring(data)
    msg_type = data.find('MsgType').text
    if msg_type == 'text':
        return TextMsg(data)
    elif msg_type == 'image':
        return ImageMsg(data)

class Msg(object):
    def __init__(self, data):
        self.ToUserName = data.find('ToUserName').text
        self.FromUserName = data.find('FromUserName').text
        self.CreateTime = data.find('CreateTime').text
        self.MsgType = data.find('MsgType').text
        self.MsgId = data.find('MsgId').text

class TextMsg(Msg):
    def __init__(self, data):
        Msg.__init__(self, data)
        self.Content = data.find('Content').text.encode('utf-8')

class ImageMsg(Msg):
    def __init__(self, data):
        Msg.__init__(self, data)
        self.PicUrl = data.find('PicUrl').text
        self.MediaId = data.find('MediaId').text

if __name__ == '__main__':
    print(parse_xml(open('text_msg.xml').read()))
    print(parse_xml(open('image_msg.xml').read()))
