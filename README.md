# Pass_ball
Pass_ball is message queue written by rust. Pass_ball has two mode to use,one is traditional sub-pub,another is req-resp like http server. The features of pass include:

- No client: All functions are realised by http api, see python requests example in test directory.
 
 - No size limit: Pass can be use to send file between devices with no fixed ip.
 
 - Req-resp mode: In this mode ,Pass will feels like a synchronise http server. This request-response mode is useful among  iot devices with no fix ip.  
 
 - Pub-Sub mod: control by api "/pass/conf", messages could be consumed "once" or "more",in "timeout" scope, messages beyond "len" will be deleted.
 
 - Persisitent data by config: Pass will clean data beyond time scope specified by sub-conf api. Therefore messages to subscriber might be duplicate.
 
 - User flexibility: it is the duty of end users to delete useful topics.  

One example for use scenario is remote_shell (https://github.com/wangmarkqi/remote_shell).
## Quich Start
See test directory for guidance.

1. pub-sub mode: 
   - cargo run main.rs
   - python pub.py
   - python sub.py
2. req-resp mode:
   - cargo run main.rs
   - python req.py
   - python resp.py
   
## .env Conf Example

```
URL=127.0.0.1:8884
ROOT=D://myrust/pass
SLEDDIR=${ROOT}/data/db
```