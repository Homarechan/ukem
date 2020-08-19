from .ukemcore import hello

__all__ = [
    "hello",
]

def sayhello(name: str) -> str:
    return hello(name)