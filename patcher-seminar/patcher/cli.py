"""Command-line interface for building, applying, and verifying patch bundles."""

import argparse
import logging
from pathlib import Path
from .builder import make_patch
from .applier import apply_patch, PatchError

def configure_logging(verbosity: int) -> None:
    """Set up global logging based on how many -v flags were supplied."""
    level = logging.WARNING  # default: only show warnings/errors to keep output quiet
    if verbosity == 1:
        level = logging.INFO  # one -v means show high-level progress messages
    elif verbosity >= 2:
        level = logging.DEBUG  # two or more -v opens up detailed debug traces
    # basicConfig wires up the root logger once, setting both level and message format
    logging.basicConfig(level=level, format="%(levelname)s %(name)s: %(message)s")

def main():
    """Parse CLI arguments and dispatch to the correct patcher operation."""
    # Top-level parser describes the tool and holds global flags such as -v / --verbose
    ap = argparse.ArgumentParser(prog="patcher", description="Simple file patcher")
    ap.add_argument(
        "-v",
        "--verbose",
        action="count",
        default=0,
        help="Increase verbosity (-v, -vv)",
    )
    # Subcommands split the functionality into make/apply/verify
    sub = ap.add_subparsers(dest="cmd", required=True)

    # "make" builds a new patch directory and manifest from a source tree
    m = sub.add_parser("make", help="Build patch from source dir")
    m.add_argument("src", type=Path)  # root folder containing files to package
    m.add_argument("out", type=Path)  # output directory that will receive the patch bundle
    m.add_argument("--version", default="1.0.0")  # version label written into the manifest
    m.add_argument(
        "--target-root",
        default="project",
        help="Logical target root name in manifest",
    )
    m.add_argument(
        "--prev-manifest",
        type=Path,
        help="Previous manifest to auto-detect deletions",
    )

    # "apply" writes a patch onto a target filesystem location (with optional backups)
    a = sub.add_parser("apply", help="Apply patch to target")
    a.add_argument("patch_dir", type=Path)  # folder containing manifest.json and payload files
    a.add_argument("target", type=Path)     # root directory that should receive the patch
    a.add_argument("--verify-only", action="store_true")  # dry run that stops after validation
    a.add_argument("--backup-dir", type=Path, help="Override backup directory")
    a.add_argument(
        "--keep-backup",
        action="store_true",
        help="Keep backup after success",
    )

    # "verify" reuses apply logic but stops after validating hashes and sizes
    v = sub.add_parser("verify", help="Verify patch payloads without writing")
    v.add_argument("patch_dir", type=Path)

    # Parse the command-line input into a structured Namespace for dispatch
    args = ap.parse_args()
    configure_logging(args.verbose)
    try:
        if args.cmd == "make":
            # Build a patch bundle using the provided source directory and options
            make_patch(
                args.src,
                args.out,
                args.version,
                target_root=args.target_root,
                prev_manifest=args.prev_manifest,
            )
        elif args.cmd == "apply":
            # Apply a patch to the target directory, optionally backing up files first
            apply_patch(
                args.patch_dir,
                args.target,
                verify_only=args.verify_only,
                backup_dir=args.backup_dir,
                keep_backup=args.keep_backup,
            )
        elif args.cmd == "verify":
            # Run the same verification logic as apply but against a dummy target root
            apply_patch(args.patch_dir, Path("."), verify_only=True)
    except PatchError as e:
        # Convert domain-specific errors into a clean process exit and clear log message
        logging.getLogger(__name__).error("ERROR: %s", e)
        raise SystemExit(1)

if __name__ == "__main__":
    main()
