import requests
from conf import host
def conf_sub():
    url = f"{host}/conf?topic=mytop&consume=more&len=16&timeout=11"
    req = requests.get(url)
    print(req.text)
def get_topic():
    url = f"{host}/topics"
    req = requests.get(url)
    print(req.text)


def del_topic():
    url = f"{host}/del/mytop"
    req = requests.get(url)
    print(req.text)


def sub():
    url = f"{host}/sub/mytop"

    res=requests.get(url)
    print (res.text)
if __name__ == '__main__':
    sub()

