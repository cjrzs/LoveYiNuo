import json

import requests

from bs4 import BeautifulSoup


BASE_URL = 'https://www.ghostoact.com'


def open_url(url):
    cookies = 'UM_distinctid=17fee4462fe78-083b3e60cd56ad-9771a39-384000-17fee4462ff3aa; CNZZDATA1259919941=88418505-1628524571-%7C1650716370; PHPSESSID=66uo6g7kb2ng1nq1ioi56438o6; Hm_lvt_ee2c341950b25fa430de1b29addb2480=1652789054; Hm_lpvt_ee2c341950b25fa430de1b29addb2480=1652843488'
    response = requests.request('GET', f'{BASE_URL}{url}', headers={'Cookie': cookies})
    return response


def get_hero_list():
    try:
        with open("hero_list.json", "r", encoding='utf-8') as f:
            hero_info = json.load(f)
    except FileNotFoundError:
        hero_list_url = f'/static/arts/json/champion.json'  # 拿到英雄列表
        response = open_url(hero_list_url)
        with open("hero_list.json", "w") as f:
            json.dump(response.text, f, ensure_ascii=False)
        hero_info = json.loads(response.text)
    return list(hero_info['champion'].keys())


def get_once_hero(hero: str=None):
    # url = f'https://www.ghostoact.com/arts/serach/?wd={{ hero }}'
    url = f'/arts/serach/?wd=亚托克斯'
    response = open_url(url)
    # print(response.text)
    new_images = {}
    soup = BeautifulSoup(response.text)
    images = soup.findAll(name='div', attrs={'class': 'mod-i'})
    for image in images:
        a = image.find_next('a')
        new_images[a.attrs['title']] = a.attrs['href']


def get_once_image():
    url = '/arts/models/?cp=266&sk=21'
    response = open_url(url)
    print(response.text)



if __name__ == '__main__':
    url = 'https://www.ghostoact.com/arts'
    url2 = 'https://www.ghostoact.com/static/common/js/common.js?v=202205042108'

    # print(open_url(url))
    # print(open_url(url2))
    # print(open_url(url3))
    # print(get_hero_list())
    # print(get_hero_list())
    # get_once_hero()
    get_once_image()




