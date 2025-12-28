import logging
from pathlib import Path, PurePosixPath
from .hashing import file_sha256
from .manifest import Manifest, ManifestEntry

log = logging.getLogger(__name__)

DEFAULT_EXCLUDES = {".git", "__pycache__", ".DS_Store"}

def should_exclude(path: Path) -> bool:
    parts = set(part.lower() for part in path.parts)
    return any(ex in parts for ex in DEFAULT_EXCLUDES)

def build_manifest(src_root: Path, version: str, target_root="project", prev_manifest: Path | None = None, deletions=None) -> Manifest:
    entries = []
    deletions = deletions or []

    # Current files
    current_paths = set()
    for path in src_root.rglob("*"):
        if path.is_file() and not should_exclude(path):
            rel = PurePosixPath(path.relative_to(src_root).as_posix())
            current_paths.add(rel)
            entries.append(
                ManifestEntry(
                    path=rel,
                    size=path.stat().st_size,
                    sha256=file_sha256(path),
                )
            )

    # Auto-detect deletions by diffing against previous manifest
    if prev_manifest:
        prev = Manifest.load(prev_manifest)
        prev_paths = {f.path for f in prev.files if f.mode == "file"}
        missing = prev_paths - current_paths
        for rel in missing:
            entries.append(ManifestEntry(path=rel, size=0, sha256="", mode="delete"))
            log.info("Detected deletion: %s", rel)

    # Explicit deletions (if provided)
    for rel in deletions:
        rel_pp = PurePosixPath(rel)
        if rel_pp not in current_paths:
            entries.append(ManifestEntry(path=rel_pp, size=0, sha256="", mode="delete"))
            log.info("Explicit deletion: %s", rel_pp)

    return Manifest(version=version, target_root=target_root, files=entries)

def make_patch(src_root: Path, out_dir: Path, version: str, target_root="project", prev_manifest: Path | None = None, deletions=None) -> None:
    src_root = src_root.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    log.info("Building manifest from %s", src_root)
    manifest = build_manifest(src_root, version, target_root=target_root, prev_manifest=prev_manifest, deletions=deletions)
    manifest.dump(out_dir / "manifest.json")
    log.info("Manifest written to %s", out_dir / "manifest.json")

    for entry in manifest.files:
        if entry.mode == "delete":
            continue  # no payload to copy
        src = src_root / Path(entry.path.as_posix())
        dst = out_dir / entry.path.as_posix()
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_bytes(src.read_bytes())
        log.debug("Copied %s -> %s", src, dst)
