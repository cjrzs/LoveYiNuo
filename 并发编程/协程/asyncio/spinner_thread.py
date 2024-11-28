import asyncio
import itertools
import sys


async def spin(msg):
    write, flush = sys.stdout.write, sys.stdout.flush
    for char in itertools.cycle('|/-\\'):
        status = f'{char} {msg}'
        write(status)
        flush()
        write('\x08' * len(status))
        try:
            await asyncio.sleep(1)
        except asyncio.CancelledError:
            break
    write(' ' * len(status) + '\x08' * len(status))


async def slow_function():
    await asyncio.sleep(3)
    return 42


async def supervisor():
    spinner = asyncio.create_task(spin('thinking!'))
    print(f'spinner object: {spinner}')
    result = await slow_function()
    spinner.cancel()
    return result


if __name__ == '__main__':
<<<<<<< HEAD
    result = asyncio.run(supervisor())
    # loop = asyncio.get_event_loop()
    # result = loop.run_until_complete(supervisor())
    # loop.close()
=======
    # loop = asyncio.get_event_loop()
    # result = loop.run_until_complete(supervisor())
    # loop.close()
    print(supervisor())
    result = asyncio.run(supervisor())
>>>>>>> 07830e7fe61000043962e1d349b4cb92f1bf9218
    print(f'Answer: {result}')



