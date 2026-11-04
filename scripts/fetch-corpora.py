#!/usr/bin/env python3
"""Fetch pinned release inputs. Verified cached files make no network request."""
from datetime import datetime, timezone
import os
from pathlib import Path
import sys
import tempfile
import urllib.request

from corpora import corpus_root, digest, sources


def fetch(source):
    root = corpus_root() / source["name"] / source["release"]
    root.mkdir(parents=True, exist_ok=True)
    downloaded = 0
    for entry in source["file"]:
        path = root / entry["name"]
        if not path.exists():
            temporary = None
            try:
                with tempfile.NamedTemporaryFile(dir=root, delete=False) as output:
                    temporary = Path(output.name)
                    with urllib.request.urlopen(entry["url"], timeout=120) as response:
                        while chunk := response.read(1024 * 1024):
                            output.write(chunk)
                if digest(temporary) != entry["sha256"]:
                    raise ValueError(f"download checksum mismatch: {entry['name']}")
                os.replace(temporary, path)
                downloaded += 1
            finally:
                if temporary is not None:
                    temporary.unlink(missing_ok=True)
        if digest(path) != entry["sha256"]:
            raise ValueError(f"cached checksum mismatch: {path}")
    readme = root / "README.md"
    if not readme.exists():
        lines = [f"# {source['name']} {source['release']}", "",
                 f"Source: {source['release_url']}",
                 f"License: {source['license']} ({source['license_url']})",
                 f"Date fetched: {datetime.now(timezone.utc).isoformat()}", "",
                 "CIViC data is dedicated to the public domain under CC0 1.0 Universal.", ""]
        for entry in source["file"]:
            lines += [f"- {entry['url']}", f"  SHA-256: {entry['sha256']}"]
        readme.write_text("\n".join(lines) + "\n")
    print(f"{source['name']} {source['release']}: {downloaded} downloaded, "
          f"{len(source['file'])} checksums verified")


def main():
    try:
        for source in sources():
            fetch(source)
    except (OSError, ValueError) as error:
        print(f"fetch corpora: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
