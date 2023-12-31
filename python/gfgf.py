# from array import array
# import reprlib
# import math


# class Vector:
#     typecode = "d"

#     def __init__(self, components):
#         self._components = array(self.typecode, components)

#     def __iter__(self):
#         return iter(self._components)

#     def __repr__(self):
#         components = reprlib.repr(self._components)
#         components = components[components.find("[") : -1]
#         return "Vector({})".format(components)

#     def __str__(self):
#         return str(tuple(self))

#     def __bytes__(self):
#         return bytes([ord(self.typecode)]) + bytes(self._components)

#     def __eq__(self, other):
#         return tuple(self) == tuple(other)

#     def __abs__(self):
#         return math.sqrt(sum(x * x for x in self))

#     def __bool__(self):
#         return bool(abs(self))

#     @classmethod
#     def frombytes(cls, octets):
#         typecode = chr(octets[0])
#         memv = memoryview(octets[1:]).cast(typecode)
#         return cls(memv)


# bt = bytes(Vector(range(10)))

# print(bt)

# print(Vector.frombytes(bt))

# import itertools

# # Lists to be chained together
# list1 = [1, 2, 3]
# list2 = ["a", "b", "c"]
# tuple1 = ("x", "y", "z")

# # Creating an iterator that chains these lists together
# chained_iter = itertools.chain(list1, list2, tuple1)

# print([x for x in chained_iter])

from collections.abc import Iterator
import functools
import time
from typing import Any

# Example usage:


class LoggedClass:
    def __getattribute__(self, attr):
        method = super().__getattribute__(attr)
        if callable(method):

            @functools.wraps(method)
            def logged_method(*args, **kwargs):
                start_time = time.time()
                result = method(*args, **kwargs)
                end_time = time.time()
                elapsed_time = end_time - start_time
                log_message = f"*** {self.__class__.__name__} CALLED '{attr}' | ARGS: {args} | KWARGS: {kwargs}. "
                log_message += f"TIME: {elapsed_time:.3f}s. ***"
                print(log_message)
                return result

            return logged_method
        return method


# Example usage:


# class MyClass(LoggedClass):
#     def method1(self, a, b):
#         [x for x in range(1000000)]
#         return 0

#     def method2(self, x, y):
#         a = []
#         for t in range(1000000):
#             a.append(t)
#         return x * y

#     def __call__(self, *args: Any, **kwds: Any) -> Any:
#         return super().__call__(*args, **kwds)


# # Creating an instance of MyClass
# obj = MyClass()

# print(callable(obj))

# # Calling methods will trigger the extra log
# result1 = obj.method1(
#     2, 3
# )  # Output: Calling method 'method1' with arguments: (2, 3), {}

# result2 = obj.method2(
#     4, 5
# )  # Output: Calling method 'method2' with arguments: (4, 5), {}


# import time


# def chain(*iterables):
#     for it in iterables:
#         print(it)
#         yield it


# s = "ABC"
# t = tuple(range(3))
# b = list(chain(s, t))
# print(b)

import asyncio


# Placeholder functions representing API calls
async def api_call1(request):
    # Simulating an asynchronous API call
    await asyncio.sleep(2)  # Simulating a delay
    return f"Response from api_call1({request})"


async def api_call2(request):
    # Simulating another asynchronous API call
    await asyncio.sleep(2)  # Simulating a delay
    return f"Response from api_call2({request})"


async def api_call3(request):
    # Simulating the third asynchronous API call
    await asyncio.sleep(2)  # Simulating a delay
    return f"Response from api_call3({request})"


# Placeholder functions representing steps in processing
def step1(response):
    # Simulating processing step 1
    return f"Step 1 processed: {response}"


def step2(response):
    # Simulating processing step 2
    return f"Step 2 processed: {response}"


def step3(response):
    # Simulating processing step 3
    print(f"Step 3 processed: {response}")
    # Do something with the final response


# The coroutine handling the three stages
async def three_stages(request1):
    response1 = await api_call1(request1)
    # stage 1
    request2 = step1(response1)
    response2 = await api_call2(request2)
    # stage 2
    request3 = step2(response2)
    response3 = await api_call3(request3)
    # stage 3
    step3(response3)


# Run the event loop with an example request
if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    request1_example = "Example Request"
    loop.run_until_complete(three_stages(request1_example))
    loop.close()
