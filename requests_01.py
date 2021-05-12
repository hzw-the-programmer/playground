import requests

resp = requests.get('http://localhost')
print(resp.text)

# r = requests.get('https://api.github.com/events', stream=True)
# print(r.raw)

# r = requests.get('https://api.github.com/events')
# print(r.content)
# print(r.text)
# print(r.json())

# payload = {'key1': 'value1', 'key2': 'value2'}
# r = requests.get('http://httpbin.org/get', params=payload)
# print(r.url)

# payload = {'key1': 'value1', 'key2': ['value2', 'value3']}
# r = requests.get('http://httpbin.org/get', params=payload)
# print(r.url)
