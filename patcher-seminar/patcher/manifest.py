"""Utilities for describing patch contents in a manifest file and validating them."""

import json
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import List

# Allowed operations recorded in the manifest; anything else is rejected up front.
ALLOWED_MODES = {"file", "delete"}

@dataclass
class ManifestEntry:
    """Represents a single path in the patch with metadata about how to handle it."""
    path: PurePosixPath  # normalized POSIX path so patches behave consistently across OSes
    size: int            # expected byte size of the payload; 0 for deletions
    sha256: str          # expected SHA-256 hex digest of the payload
    mode: str = "file"   # "file" means write/replace, "delete" means remove at apply time

@dataclass
class Manifest:
    """In-memory model of an entire manifest, including version, root, and file entries."""
    version: str         # version label associated with this patch (opaque to the tool)
    target_root: str     # top-level folder name on the target where files should land
    files: List[ManifestEntry]  # ordered list of file or deletion operations

    @staticmethod
    def load(path: Path) -> "Manifest":
        """Read a manifest JSON file from disk and convert it into Manifest objects."""
        data = json.loads(Path(path).read_text(encoding="utf-8"))
        files = []
        for f in data["files"]:
            p = PurePosixPath(f["path"])  # normalize separators and block Windows-style backslashes
            if ".." in p.parts:
                # Reject attempts to escape the target root when applying the patch
                raise ValueError(f"Illegal path traversal in manifest: {p}")
            mode = f.get("mode", "file")
            if mode not in ALLOWED_MODES:
                # Fail fast if a manifest refers to an unsupported operation
                raise ValueError(f"Unsupported mode in manifest: {mode}")
            files.append(
                ManifestEntry(
                    path=p,
                    size=int(f["size"]),          # ensure numeric type even if JSON stored as string
                    sha256=str(f["sha256"]),      # always treat digest as text
                    mode=f.get("mode", "file"),   # allow older manifests without mode field
                )
            )
        return Manifest(
            version=str(data["version"]),         # cast for defensive typing
            target_root=str(data["target_root"]), # store root as plain string for JSON compatibility
            files=files,
        )

    def dump(self, path: Path) -> None:
        """Serialize the manifest back to JSON and write it to the given path."""
        data = {
            "version": self.version,
            "target_root": self.target_root,
            "files": [
                {
                    "path": str(f.path),  # convert PurePosixPath back to JSON-friendly string
                    "size": f.size,
                    "sha256": f.sha256,
                    "mode": f.mode,
                }
                for f in self.files
            ],
        }
        path.write_text(json.dumps(data, indent=2), encoding="utf-8")
