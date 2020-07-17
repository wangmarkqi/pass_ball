import requests
import time
from conf import host


def pub():
    url = f"{host}/pub/mytop"

    start = 230
    d = dict(
        only=str(start),
    )
    res = requests.post(url, json=d)
    print(res.text)

if __name__ == '__main__':
    pub()
