#!/usr/bin/env python3
import requests

from conf import host

def req():
    url = f"{host}/req/tianjin_door/10"
    req=requests.post(url,json="close")
    print (req.text)


if __name__ == '__main__':
    req()


