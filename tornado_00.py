import os
import tornado.template
import tornado.ioloop
import tornado.web

class Form01Handler(tornado.web.RequestHandler):
    def prepare(self):
        pass

    def get(self):
        self.render('form01.html')

    def post(self):
        username = self.get_body_argument('username')
        password = self.get_body_argument('password')
        self.write('your name: %s your password: %s' % (username, password))

app = tornado.web.Application(
    [
        (r'/form01', Form01Handler)
    ],
    static_path=os.path.join(os.path.dirname(__file__), 'static'),
    template_path=os.path.join(os.path.dirname(__file__), 'templates'))
app.listen(80)
tornado.ioloop.IOLoop.current().start()
