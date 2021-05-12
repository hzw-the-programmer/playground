import requests

# resp = requests.get('http://localhost')
# print(resp.text)

resp = requests.get('http://localhost/story/1')
print(resp.text)

# from pretty_print_req import pretty_print_req

# payload = {'key1': 'value1', 'key2': 'value2'}
# payload = {'key1': 'value1', 'key2': ['value2', 'value3']}
# req = requests.Request('GET', 'http://httpbin.org/get', params=payload)
# prepared = req.prepare()
# pretty_print_req(prepared)

# cookies = dict(cookies_are='working')
# jar = requests.cookies.RequestsCookieJar()
# jar.set('tasty_cookie', 'yum', domain='httpbin.org', path='/cookies')
# jar.set('gross_cookie', 'blech', domain='httpbin.org', path='/elsewhere')
# req = requests.Request('GET', 'http://httpbin.org/cookies', cookies=cookies)
# req = requests.Request('GET', 'http://httpbin.org/cookies', cookies=jar)
# prepared = req.prepare()
# pretty_print_req(prepared)

# payload = {'key1': 'value1', 'key2': 'value2'}
# payload = (('key1', 'value1'), ('key1', 'value2'))
# req = requests.Request('POST', 'http://httpbin.org/post', data=payload)
# req = requests.Request('POST', 'http://httpbin.org/post', json=payload)
# prepared = req.prepare()
# pretty_print_req(prepared)

# files = {'file': open('requests_02.py', 'rb')}
# files = {'file': ('requests02.py', open('requests_02.py', 'rb'), 'application/vnd.ms-excel', {'Expires': '0'})}
# files = {'file': ('report.csv', 'some,data,to,send\nanother,row,to,send\n')}
# req = requests.Request('POST', 'http://httpbin.org/post', files=files)
# prepared = req.prepare()
# pretty_print_req(prepared)
