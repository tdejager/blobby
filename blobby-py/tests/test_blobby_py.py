from blobby_py import __version__
import aiohttp
import asyncio
import msgpack
import time
import typing

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

def create_metadata() -> typing.Dict[str, typing.Any]:
    return {
        "file_name": "foo", "extension" : "bar", "tags" : [], "timestamp": round(time.time() * 1000)
    }

def test_msgpack_metadata():
    """Write metadata msg from python"""
    data = msgpack.packb(create_metadata(), use_bin_type=True)
    if data:
        file = open("/tmp/metadata.msgpack", "wb")
        file.write(data)
        file.close()

def test_msgpack_message():
    """Write blob message from python"""
    blob = {
        "metadata": create_metadata(), "data": bytearray([0, 1, 2, 3])
    }
    data = msgpack.packb(blob, use_bin_type=True)
    if data:
        file = open("/tmp/blob.msgpack", "wb")
        file.write(data)
        file.close()
