from http.server import HTTPServer
from http.server import CGIHTTPRequestHandler

class CGIHTTPRequestHandler1(CGIHTTPRequestHandler):
    def handle_one_request(self):
        self.log_message('handle_one_request begin')
        CGIHTTPRequestHandler.handle_one_request(self)
        self.log_message('handle_one_request end')

httpserver = HTTPServer(('', 8888), CGIHTTPRequestHandler1)
httpserver.serve_forever()
