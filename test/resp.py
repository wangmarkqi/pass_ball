#!/usr/bin/env python3
import requests
import json

from conf import host

def hb():
    url = f"{host}/hb/tianjin_door"

    while True:
        req=requests.get(url)
        res=req.text
        print (res)
        if res!="" and "close" in res:
            print (req.json())
            resp()


def resp():
    url = f"{host}/resp/tianjin_door"
    data = dict(
        asdfa=987,
        pro="adfasdfa"
    )
    res=requests.post(url, json=data)
    print ("resp result===",res.text)

if __name__ == '__main__':
    hb()
