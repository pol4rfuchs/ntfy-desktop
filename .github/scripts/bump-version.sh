#!/usr/bin/env bash

# #
#   Raises the app version in every place that holds it.
#
#   usage:  bump-version.sh <patch|minor|major|none> [repo-root]
#
#   Files:  src-tauri/tauri.conf.json, src-tauri/Cargo.toml, src-tauri/Cargo.lock
#           and the file names in the README download examples.
#
#   stdout: the version after the bump (only that, so it can be captured).
#   Everything else goes to stderr. With `none` nothing is changed.
# #

set -euo pipefail

BUMP="${1:?usage: bump-version.sh <patch|minor|major|none> [repo-root]}"
ROOT="${2:-.}"

CONF="$ROOT/src-tauri/tauri.conf.json"
CARGO="$ROOT/src-tauri/Cargo.toml"
LOCK="$ROOT/src-tauri/Cargo.lock"
README="$ROOT/README.md"

OLD=$(sed -n 's/^[[:space:]]*"version":[[:space:]]*"\([^"]*\)".*/\1/p' "$CONF" | head -n 1)
CARGO_VER=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$CARGO" | head -n 1)

if [ "$OLD" != "$CARGO_VER" ]; then
    echo "::error::tauri.conf.json ($OLD) and Cargo.toml ($CARGO_VER) have different versions" >&2
    exit 1
fi
if ! [[ "$OLD" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)$ ]]; then
    echo "::error::version '$OLD' is not in the form X.Y.Z" >&2
    exit 1
fi
MAJOR="${BASH_REMATCH[1]}"; MINOR="${BASH_REMATCH[2]}"; PATCH="${BASH_REMATCH[3]}"

case "$BUMP" in
    patch) NEW="$MAJOR.$MINOR.$((PATCH + 1))" ;;
    minor) NEW="$MAJOR.$((MINOR + 1)).0" ;;
    major) NEW="$((MAJOR + 1)).0.0" ;;
    none)  NEW="$OLD" ;;
    *) echo "::error::unknown bump '$BUMP' (use patch, minor, major or none)" >&2; exit 1 ;;
esac

if [ "$NEW" != "$OLD" ]; then
    echo "Version $OLD -> $NEW" >&2

    sed -i "0,/\"version\": \"$OLD\"/s//\"version\": \"$NEW\"/" "$CONF"
    sed -i "0,/^version = \"$OLD\"/s//version = \"$NEW\"/" "$CARGO"

    # the lock file lists the app itself with its version
    python3 - "$LOCK" "$OLD" "$NEW" <<'PY'
import sys
path, old, new = sys.argv[1:4]
text = open(path, encoding="utf-8").read()
needle = 'name = "ntfy-desktop"\nversion = "%s"' % old
if text.count(needle) != 1:
    sys.exit("Cargo.lock: entry for ntfy-desktop %s not found" % old)
open(path, "w", encoding="utf-8", newline="\n").write(text.replace(needle, needle.replace(old, new)))
PY

    # file names in the README download examples
    sed -i "s/ntfy-desktop-$OLD-/ntfy-desktop-$NEW-/g" "$README"
else
    echo "Version stays at $OLD" >&2
fi

echo "$NEW"
