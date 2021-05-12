import tornado.template

t = tornado.template.Template('<html>{{ myvalue }}</html>')
print(t.generate(myvalue='We loves You!'))
