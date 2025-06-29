import cv2
import numpy as np
import asyncio
import nats

async def main():
    nc = await nats.connect("nats://localhost:4222")

    async def message_handler(msg):
        data = msg.data
        # Decode JPEG bytes to image
        img = cv2.imdecode(np.frombuffer(data, dtype=np.uint8), cv2.IMREAD_COLOR)
        if img is not None:
            cv2.imshow("Received Frame", img)
            if cv2.waitKey(1) & 0xFF == ord('q'):
                exit(0)

    await nc.subscribe("camera.kitchen", cb=message_handler)
    await asyncio.Event().wait()

if __name__ == '__main__':
    asyncio.run(main())
