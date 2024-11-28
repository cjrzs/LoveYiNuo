import time
import aiohttp
import asyncio
import requests

async def fetch(client, number):
    async with client.request('POST', 'http://127.0.0.1:8080', params={'number': number}) as resp:
        return await resp.text()
async def main():
    tasks = []
    async with aiohttp.ClientSession() as client:
        for i in range(3):
            tasks.append(fetch(client, i))
        done, pending = await asyncio.wait(tasks)
        for res in done:
            print(res.result())

start = time.time()
for i in range(3):
    res = requests.request(
        'POST',
        'http://127.0.0.1:8080',
        params={'number': i}
    )
    print(res.text)
print(time.time() - start)
start = time.time()
asyncio.run(main())
print(time.time() - start)

