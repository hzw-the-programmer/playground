from SOAPpy import SOAPProxy

url = 'http://wechat.kaifa.cn/wechatTest/openService/alamService.asmx'
server = SOAPProxy(url)
server.KedasAlam('')
