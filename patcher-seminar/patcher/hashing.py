from hashlib import sha256
from pathlib import Path

BUF_SIZE = 1024 * 1024  # 1 MiB chunks to keep memory low

def file_sha256(path: Path) -> str:
    """Return the SHA-256 hex digest of a file using buffered reads."""
    h = sha256()
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(BUF_SIZE), b""):
            h.update(chunk)
    return h.hexdigest()
