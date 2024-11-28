# import asyncio
#
# async def main():
#     print('hello')
#     await asyncio.sleep(1)
#     print('world')
#
# asyncio.run(main())
import dis


class Test:
    __attr1 = 'QAQ'

    def qqq(self):
        print(self.__attr1)

t = Test()
t.qqq()
# print(t._Test__attr1)

