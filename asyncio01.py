import asyncio

@asyncio.coroutine
def bug():
    raise Exception("not consumed")

bug()
