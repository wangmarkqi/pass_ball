#!/usr/bin/env python3
import requests
import json

from conf import host

def hb():
    url = f"{host}/hb/mytop"

    while True:
        req=requests.get(url)
        if req.text!="":
            d=json.loads(req.text)
            print (d)
            resp()


def resp():
    url = f"{host}/resp/mytop"
    data = dict(
        asdfa=987,
        pro="adfasdfa"
    )
    res=requests.post(url, json=data)
    print ("resp result===",res.text)

if __name__ == '__main__':
    hb()
    # resp("mytop")
