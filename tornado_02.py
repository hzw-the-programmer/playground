import tornado.ioloop
import tornado.web

class MainHandler(tornado.web.RequestHandler):
    def get(self):
        self.write('<a href="%s">link to story 1</a>' %
                    self.reverse_url('story', 1))

class StoryHandler(tornado.web.RequestHandler):
    def initialize(self, db):
        self.db = db

    def get(self, story_id):
        self.write('db is %s, story is %s' % (self.db, story_id))

app = tornado.web.Application([
    tornado.web.url(r'/', MainHandler),
    tornado.web.url(r'/story/([0-9]+)', StoryHandler, dict(db='db'), name='story')
])
app.listen(80)
tornado.ioloop.IOLoop.current().start()
