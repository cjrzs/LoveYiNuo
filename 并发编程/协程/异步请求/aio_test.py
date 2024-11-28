import asyncio
from aiohttp import web

async def handler(request):
    data = request.query
    await asyncio.sleep(1)
    return web.json_response(text=data['number'])

app = web.Application()
app.router.add_post('/', handler)
web.run_app(app)
