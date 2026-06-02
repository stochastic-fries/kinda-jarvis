import asyncio
import websockets

async def handler(websocket):
    print("ESP32 connected")
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