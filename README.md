# MochiCam

Expandable home internet camera system

# TODO

* [ ] HTTP server
* [ ] Server Discovery
* [ ] Setup Script
* [ ] Documentation

# Manuel Setup

1. Start NATS server
2. Start HTTP server
3. Give NATS server IP to your camera software
    
    EX:
    ```py
        nc = await nats.connect("nats://localhost:4222")
    ```

# PoC Demo
1. Start NATS Server
2. Do [step 3](#manuel-setup) from manuel setup in  [cam.py](./cam.py) and [testsub.py](./testsub.py)
3. Run [cam.py](./cam.py) and [testsub.py](./testsub.py) in seperate terminals