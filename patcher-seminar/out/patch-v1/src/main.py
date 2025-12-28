from .util import shout

def greet(name: str) -> str:
    return f"Hello, {name}!"

if __name__ == "__main__":
    print(shout(greet("world")))
