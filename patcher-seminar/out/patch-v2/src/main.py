from .util import shout

def greet(name: str) -> str:
    return f"Hello there, {name}! Welcome aboard."

def new_feature() -> str:
    return "This is a new feature."

if __name__ == "__main__":
    print(shout(greet("world")))
    print(new_feature())
# corruption
