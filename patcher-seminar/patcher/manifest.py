import json
from dataclasses import dataclass
from pathlib import Path, PurePosixPath
from typing import List

ALLOWED_MODES = {"file", "delete"}

@dataclass
class ManifestEntry:
    path: PurePosixPath # PurePosixPath to also work on systems other than windows
    size: int
    sha256: str
    mode: str = "file"  # future-proof; only files for now

@dataclass
class Manifest:
    version: str
    target_root: str
    files: List[ManifestEntry]

    @staticmethod
    def load(path: Path) -> "Manifest":
        data = json.loads(Path(path).read_text(encoding="utf-8"))
        files = []
        for f in data["files"]:
            p = PurePosixPath(f["path"])
            if ".." in p.parts:
                raise ValueError(f"Illegal path traversal in manifest: {p}")
            mode = f.get("mode", "file")
            if mode not in ALLOWED_MODES:
                raise ValueError(f"Unsupported mode in manifest: {mode}")
            files.append(
                ManifestEntry(
                    path=p,
                    size=int(f["size"]),
                    sha256=str(f["sha256"]),
                    mode=f.get("mode", "file"),
                )
            )
        return Manifest(
            version=str(data["version"]),
            target_root=str(data["target_root"]),
            files=files,
        )

    def dump(self, path: Path) -> None:
        data = {
            "version": self.version,
            "target_root": self.target_root,
            "files": [
                {
                    "path": str(f.path),
                    "size": f.size,
                    "sha256": f.sha256,
                    "mode": f.mode,
                }
                for f in self.files
            ],
        }
        path.write_text(json.dumps(data, indent=2), encoding="utf-8")
