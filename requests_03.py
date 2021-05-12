import requests

data = {'message': 'I love turnado!'}
resp = requests.post('http://localhost/myform', data=data)
print(resp.text)
