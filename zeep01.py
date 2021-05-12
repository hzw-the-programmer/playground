from zeep import Client

client = Client('http://wechat.kaifa.cn/wechatTest/openService/alamService.asmx')
result = client.server.KedasAlam('')
print(result)
