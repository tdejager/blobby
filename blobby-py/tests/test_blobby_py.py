from blobby_py import __version__
import aiohttp
import asyncio
import msgpack
import time


def test_version():
    assert __version__ == '0.1.0'

def test_http_request():
    async def do_request():
        async with aiohttp.ClientSession() as session:
            async with session.get('http://python.org') as response:

                print("Status:", response.status)
                print("Content-type:", response.headers['content-type'])

                html = await response.text()
                print("Body:", html[:15], "...")
    loop = asyncio.get_event_loop()
    loop.run_until_complete(do_request())

def test_msgpack_serialize():
    metadata = {
        "file_name": "foo", "extension" : "bar", "tags" : [], "timestamp": round(time.time() * 1000)
    }
    data = msgpack.packb(metadata, use_bin_type=True)
    if data:
        file = open("/tmp/metadata.msgpack", "wb")
        file.write(data)
        file.close()


