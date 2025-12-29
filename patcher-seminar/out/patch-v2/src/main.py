from .util import shout
from .feature import second_feature

def greet(name: str) -> str:
    return f"Hello there, {name}! Welcome aboard."

def new_feature() -> str:
    return "This is a old feature."

if __name__ == "__main__":
    print(shout(greet("world")))
    print(new_feature())
    print(second_feature())
