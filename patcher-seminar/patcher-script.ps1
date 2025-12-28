# Make sure both src trees are packages
New-Item -ItemType File simple-project\src\__init__.py -Force | Out-Null
New-Item -ItemType File simple-project-target\src\__init__.py -Force | Out-Null

# Build v2 using v1 manifest for deletions
python -m patcher.cli -v make simple-project out/patch-v2 --version 1.1.0 --prev-manifest out/patch-v1/manifest.json

# Apply to the target copy
python -m patcher.cli -v apply out/patch-v2 simple-project-target

# Run the app from inside the target so the relative import works
Push-Location simple-project-target
python -m src.main
Pop-Location
