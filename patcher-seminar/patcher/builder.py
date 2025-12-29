"""Builds patch manifests and payload directories from a source tree."""

import logging
from pathlib import Path, PurePosixPath
from .hashing import file_sha256
from .manifest import Manifest, ManifestEntry

log = logging.getLogger(__name__)

# Files/directories to ignore when scanning the source tree; avoids shipping metadata or caches.
DEFAULT_EXCLUDES = {".git", "__pycache__", ".DS_Store"}

def should_exclude(path: Path) -> bool:
    """Case-insensitive check for entries we do not want to include in a patch."""
    parts = set(part.lower() for part in path.parts)  # examine every segment of the path
    return any(ex in parts for ex in DEFAULT_EXCLUDES)  # true once any banned name is present

def build_manifest(src_root: Path, version: str, target_root="project", prev_manifest: Path | None = None, deletions=None) -> Manifest:
    """Walk src_root and produce a manifest including inferred and explicit deletions."""
    entries = []  # accumulated manifest entries in the order discovered
    deletions = deletions or []  # normalize None to empty list so loops are straightforward

    # Current files
    current_paths = set()  # track current files to compare against previous manifest
    for path in src_root.rglob("*"):
        if path.is_file() and not should_exclude(path):
            rel = PurePosixPath(path.relative_to(src_root).as_posix())  # relative POSIX path for portability
            current_paths.add(rel)  # remember this path for deletion detection
            entries.append(
                ManifestEntry(
                    path=rel,
                    size=path.stat().st_size,     # record size now to catch changes later
                    sha256=file_sha256(path),     # hash content so applier can verify integrity
                )
            )

    # Auto-detect deletions by diffing against previous manifest
    if prev_manifest:
        prev = Manifest.load(prev_manifest)  # reuse parsing/validation logic
        prev_paths = {f.path for f in prev.files if f.mode == "file"}  # ignore prior deletions
        missing = prev_paths - current_paths  # anything not present anymore is a delete
        for rel in missing:
            entries.append(ManifestEntry(path=rel, size=0, sha256="", mode="delete"))
            log.info("Detected deletion: %s", rel)

    # Explicit deletions (if provided)
    for rel in deletions:
        rel_pp = PurePosixPath(rel)  # normalize to match stored manifest paths
        if rel_pp not in current_paths:
            entries.append(ManifestEntry(path=rel_pp, size=0, sha256="", mode="delete"))
            log.info("Explicit deletion: %s", rel_pp)

    return Manifest(version=version, target_root=target_root, files=entries)

def make_patch(src_root: Path, out_dir: Path, version: str, target_root="project", prev_manifest: Path | None = None, deletions=None) -> None:
    """Create a patch directory containing a manifest and payload files."""
    src_root = src_root.resolve()  # resolve early so copied paths are absolute and consistent
    out_dir.mkdir(parents=True, exist_ok=True)  # ensure destination exists before writing anything
    log.info("Building manifest from %s", src_root)
    manifest = build_manifest(src_root, version, target_root=target_root, prev_manifest=prev_manifest, deletions=deletions)  # gather entries including deletions
    manifest.dump(out_dir / "manifest.json")
    log.info("Manifest written to %s", out_dir / "manifest.json")

    for entry in manifest.files:
        if entry.mode == "delete":
            continue  # no payload to copy for deletions
        src = src_root / Path(entry.path.as_posix())  # source file to package
        dst = out_dir / entry.path.as_posix()         # destination path inside patch folder
        dst.parent.mkdir(parents=True, exist_ok=True) # create directories for nested files
        dst.write_bytes(src.read_bytes())             # copy bytes directly; size already recorded
        log.debug("Copied %s -> %s", src, dst)
