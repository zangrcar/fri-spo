import argparse
import logging
from pathlib import Path
from .builder import make_patch
from .applier import apply_patch, PatchError

def configure_logging(verbosity: int) -> None:
    level = logging.WARNING
    if verbosity == 1:
        level = logging.INFO
    elif verbosity >= 2:
        level = logging.DEBUG
    logging.basicConfig(level=level, format="%(levelname)s %(name)s: %(message)s")

def main():
    ap = argparse.ArgumentParser(prog="patcher", description="Simple file patcher")
    ap.add_argument("-v", "--verbose", action="count", default=0, help="Increase verbosity (-v, -vv)")
    sub = ap.add_subparsers(dest="cmd", required=True)

    m = sub.add_parser("make", help="Build patch from source dir")
    m.add_argument("src", type=Path)
    m.add_argument("out", type=Path)
    m.add_argument("--version", default="1.0.0")
    m.add_argument("--target-root", default="project", help="Logical target root name in manifest")
    m.add_argument("--prev-manifest", type=Path, help="Previous manifest to auto-detect deletions")

    a = sub.add_parser("apply", help="Apply patch to target")
    a.add_argument("patch_dir", type=Path)
    a.add_argument("target", type=Path)
    a.add_argument("--verify-only", action="store_true")
    a.add_argument("--backup-dir", type=Path, help="Override backup directory")
    a.add_argument("--keep-backup", action="store_true", help="Keep backup after success")

    v = sub.add_parser("verify", help="Verify patch payloads without writing")
    v.add_argument("patch_dir", type=Path)

    args = ap.parse_args()
    configure_logging(args.verbose)
    try:
        if args.cmd == "make":
            make_patch(
                args.src,
                args.out,
                args.version,
                target_root=args.target_root,
                prev_manifest=args.prev_manifest,
            )
        elif args.cmd == "apply":
            apply_patch(
                args.patch_dir,
                args.target,
                verify_only=args.verify_only,
                backup_dir=args.backup_dir,
                keep_backup=args.keep_backup,
            )
        elif args.cmd == "verify":
            apply_patch(args.patch_dir, Path("."), verify_only=True)
    except PatchError as e:
        logging.getLogger(__name__).error("ERROR: %s", e)
        raise SystemExit(1)

if __name__ == "__main__":
    main()
