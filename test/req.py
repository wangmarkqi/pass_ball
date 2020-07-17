#!/usr/bin/env python3
import requests

from conf import host

def req():
    url = f"{host}/req/mytop/10"
    d=dict(
        name="mytop",
        data="adfa",
    )
    req=requests.post(url,json=d)
    print (req.text)


if __name__ == '__main__':
    req()


