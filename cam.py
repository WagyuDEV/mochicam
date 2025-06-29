import cv2
import asyncio
import nats

async def main():
    nc = await nats.connect("nats://localhost:4222")
    cap = cv2.VideoCapture(0)

    while True:
        ret, frame = cap.read()
        if not ret:
            continue

        # Encode frame as JPEG
        success, buffer = cv2.imencode('.jpg', frame)
        if not success:
            continue

        # Convert to bytes and send
        await nc.publish("camera.kitchen", buffer.tobytes())

        await asyncio.sleep(1/60)  # ~30 fps

    await nc.drain()

if __name__ == '__main__':
    asyncio.run(main())
