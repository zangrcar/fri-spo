__version__ = "1.0.0"

from .hashing import file_sha256
from .manifest import Manifest, ManifestEntry
from .builder import make_patch, build_manifest
from .applier import apply_patch, PatchError

__all__ = [
    "__version__",
    "file_sha256",
    "Manifest",
    "ManifestEntry",
    "make_patch",
    "build_manifest",
    "apply_patch",
    "PatchError",
]
