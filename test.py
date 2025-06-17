import asyncio


async def func(bus_name):
    print(f"{bus_name} 满员")
    await asyncio.sleep(1)
    print(f"{bus_name} 发车")


async def main():
    tasks = [
        asyncio.create_task(func("1")),
        asyncio.create_task(func("2")),
        asyncio.create_task(func("3")),
    ]
    await asyncio.gather(*tasks)


if __name__ == '__main__':
    asyncio.run(main())



