import asyncio
import websockets
import time

async def handler(websocket):
    print("ESP32 connected")
    await websocket.send(bytes([0x01, 0x01, 0x00, 0x00]))  # 
    try:
        async for message in websocket:
            print(f"received: {message}")
    except websockets.exceptions.ConnectionClosed:
        print("ESP32 disconnected")

async def main():
    print("WebSocket started on port 9090")
    async with websockets.serve(handler, "0.0.0.0", 9090):
        await asyncio.Future()  

asyncio.run(main())