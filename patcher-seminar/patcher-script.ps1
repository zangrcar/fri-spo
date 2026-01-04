# patcher-walkthrough.ps1

# 1) Make your changes for the new version in the SOURCE tree:
#    edit files under simple-project\src\ (main.py, feature.py, util.py) or README/config.
#    Do NOT touch simple-project-target; that stays as the old install we will patch.

# 2) Build a patch bundle from the updated source.
python -m patcher.cli -v make simple-project out/patch-v2 `
  --version 1.2.0 `
  --prev-manifest out/patch-v1/manifest.json

# 3) Optional safety check: verify hashes without writing to the target.
python -m patcher.cli -v apply out/patch-v2 simple-project-target --verify-only

# 4) Apply the patch to the target copy (add --keep-backup to retain backups).
python -m patcher.cli -v apply out/patch-v2 simple-project-target # --keep-backup to keep backup files

# 5) Run the app from the patched target to confirm behavior.
Push-Location simple-project-target
python -m src.main
Pop-Location
