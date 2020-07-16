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
            resp(d['topic'])


def resp(topic):
    url = "http://127.0.0.1:8084/pass/resp"
    d = dict(
        topic=topic,
        answer="133331"
    )
    res=requests.post(url, json=d)
    print ("resp result===",res.text)

if __name__ == '__main__':
    hb()
    # resp("mytop")
