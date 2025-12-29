"""Helpers for hashing patch payloads without loading entire files into memory."""

from hashlib import sha256
from pathlib import Path

# Read files in 1 MiB chunks to balance I/O efficiency with memory use.
BUF_SIZE = 1024 * 1024  # 1 MiB chunks to keep memory low

def file_sha256(path: Path) -> str:
    """Return the SHA-256 hex digest of a file using buffered reads."""
    h = sha256()  # hasher accumulates bytes incrementally
    with path.open("rb") as f:
        for chunk in iter(lambda: f.read(BUF_SIZE), b""):  # loop until an empty chunk signals EOF
            h.update(chunk)  # feed each chunk into the digest
    return h.hexdigest()  # final 64-char hex string
