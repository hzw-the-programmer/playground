import tornado.ioloop
import tornado.web

class MyFormHandler(tornado.web.RequestHandler):
    def get(self):
        self.write('<html><body><form action="/myform" method="POST">'
                   '<input type="text" name="message">'
                   '<input type="submit" value="Submit">'
                   '</form></body></html>')

    def post(self):
        self.set_header("Content-Type", "text/plain")
        self.write("You wrote: " + self.get_body_argument("message"))

app = tornado.web.Application([
    (r'/myform', MyFormHandler)
])
app.listen(80)
tornado.ioloop.IOLoop.current().start()
