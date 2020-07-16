#!/usr/bin/env python3
import requests

from conf import host

def req():
    url = f"{host}/req"
    d=dict(
        topic="mytop",
        data="adfa",
        timeout=4,
    )
    req=requests.post(url,json=d)
    print (req.text)


if __name__ == '__main__':
    req()


