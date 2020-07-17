import requests
import time
from conf import host


def pub():
    url = f"{host}/pub"

    start = 230
    d = dict(
        topic="mytop",
        answer=str(start),
    )
    res = requests.post(url, json=d)
    print(res.text)
def test():
    url = f"{host}/test/asdfa"

    start = 230
    d = dict(
        topic="mytop",
        answer=str(start),
    )
    res = requests.post(url, data=d)
    print(res.text)

if __name__ == '__main__':
    # pub()
    test()
