#!/usr/bin/env python3
import requests
from bs4 import BeautifulSoup

if __name__ == '__main__':
    url = 'https://en.wikipedia.org/wiki/Emoji'
    r = requests.get(url)

    soup = BeautifulSoup(r.text, 'lxml')
    table = soup.select_one('table.wikitable:nth-child(70) > tbody')

    output = open('emojis', 'w')
    for td in table.find_all('td'):
        try:
            e = td['title']
        except Exception:
            continue
        print(td.text.strip(), e.split(':')[1][1:].capitalize(), file=output)
