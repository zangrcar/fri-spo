"""Apply a built patch directory to a target tree with optional backups and verification."""

import logging
import shutil
import os
from pathlib import Path
from typing import Dict, List, Tuple
from .manifest import Manifest
from .hashing import file_sha256

log = logging.getLogger(__name__)

class PatchError(Exception):
    """Raised when applying a patch fails validation or file operations."""
    pass

def _ensure_under_root(root: Path, target: Path) -> None:
    """Guarantee the target path stays within the intended root directory."""
    try:
        target.resolve().relative_to(root.resolve())
    except Exception:
        raise PatchError(f"Target path escapes root: {target}")

def apply_patch(
    patch_dir: Path,
    target_root: Path,
    verify_only: bool = False,
    backup_dir: Path | None = None,
    keep_backup: bool = False,
) -> None:
    """Apply files and deletions from a patch directory into a target tree."""
    patch_dir = patch_dir.resolve()
    target_root = target_root.resolve()
    log.info("Applying patch from %s to %s", patch_dir, target_root)
    manifest = Manifest.load(patch_dir / "manifest.json")

    # Preflight: validate payloads (skip deletes)
    for entry in manifest.files:
        if entry.mode == "delete":
            continue
        payload = patch_dir / entry.path.as_posix()
        if not payload.is_file():
            raise PatchError(f"Payload missing: {payload}")
        if payload.stat().st_size != entry.size:
            raise PatchError(f"Size mismatch for {entry.path}")
        if file_sha256(payload) != entry.sha256:
            raise PatchError(f"Hash mismatch for {entry.path}")

    if verify_only:
        log.info("Verify-only complete")
        return

    backup_root = (backup_dir or (target_root / "backup")).resolve()
    staged: List[Tuple[Path, Path]] = []  # (tmp, dest) for file replacements
    backups: Dict[Path, Path] = {}        # dest -> backup
    deletes: List[Path] = []              # targets to delete

    try:
        # Stage new content to temps; do not touch live files yet
        for entry in manifest.files:
            dest = target_root / entry.path.as_posix()
            _ensure_under_root(target_root, dest)
            if entry.mode == "delete":
                deletes.append(dest)
                continue
            tmp = dest.with_suffix(dest.suffix + ".patchtmp")  # temporary file alongside destination
            tmp.parent.mkdir(parents=True, exist_ok=True)        # ensure directory exists before copying
            payload = patch_dir / entry.path.as_posix()
            shutil.copyfile(payload, tmp)                        # stage payload without affecting live file
            staged.append((tmp, dest))
            log.debug("Staged %s -> %s", payload, tmp)

        # Backup originals and commit replacements atomically
        for tmp, dest in staged:
            if dest.exists():
                backup_path = backup_root / dest.relative_to(target_root)
                backup_path.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(dest, backup_path)           # preserve current version before overwrite
                backups[dest] = backup_path                  # remember so we can roll back if needed
                log.debug("Backed up %s -> %s", dest, backup_path)
            dest.parent.mkdir(parents=True, exist_ok=True)
            os.replace(tmp, dest)  # atomic per file on Windows
            log.info("Replaced %s", dest)

        # Handle deletions
        for dest in deletes:
            if dest.exists():
                backup_path = backup_root / dest.relative_to(target_root)
                backup_path.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(dest, backup_path)  # save deleted file to backup for rollback/recovery
                backups[dest] = backup_path
                dest.unlink()
                log.info("Deleted %s", dest)

    except Exception as e:
        log.error("Apply failed: %s", e)
        # Rollback: restore from backups where present
        for dest, backup_path in backups.items():
            if backup_path.is_file():
                dest.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(backup_path, dest)
                log.warning("Restored %s from %s", dest, backup_path)
        # Clean temps
        for tmp, _dest in staged:
            if Path(tmp).exists():
                try:
                    Path(tmp).unlink()
                except OSError:
                    pass
        raise PatchError(f"Apply failed: {e}") from e
    else:
        # Success: remove temps (if any left) and optionally cleanup backups
        for tmp, _dest in staged:
            if Path(tmp).exists():
                try:
                    Path(tmp).unlink()
                except OSError:
                    pass
        if not keep_backup and backup_root.exists():
            shutil.rmtree(backup_root)
            log.info("Removed backup at %s", backup_root)
        log.info("Apply completed successfully")
