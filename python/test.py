from bs4 import BeautifulSoup
import os
import requests
import sys

def save(url, directory, referer):
    headers = {'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.78 Safari/537.36'}
    fn = os.path.join(directory, url[url.rfind('/') + 1:])
    r = requests.get(url, headers=headers, stream=True)
    if r.status_code != requests.codes.ok:
        return
    with open(fn, 'wb') as fd:
        for chunk in r.iter_content(chunk_size=128):
            fd.write(chunk)

def d1(url, directory):
    r = requests.get(url)
    soup = BeautifulSoup(r.text, 'html.parser')
    for div in soup.find_all(id='picture'):
        for img in div.find_all('img'):
            save(img.get('src'), directory, url)

url = sys.argv[1]

r = requests.get(url)
r.encoding = 'gb2312'
soup = BeautifulSoup(r.text, 'html.parser')
for div in soup.find_all(id='picture'):
    a = div.find('a')
    directory = os.path.join('pics', a.get('title'))
    os.makedirs(directory, exist_ok=True)
    d1(a.get('href'), directory)
    # break
