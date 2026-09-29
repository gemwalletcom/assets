#!/usr/bin/env python3
"""Check that token logos are square and hold no JPEG content, without modifying them."""

import argparse
import sys
from pathlib import Path

from PIL import Image


REJECTED_FORMATS = {"JPEG"}


def is_accepted(size, image_format):
    width, height = size
    return width == height and image_format not in REJECTED_FORMATS


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=Path(__file__).resolve().parents[1], help="assets repository root (defaults to this script's repository)")
    args = parser.parse_args()
    root = args.root.resolve()
    logos = sorted(root.glob("blockchains/*/assets/*/logo.png"))
    if not logos:
        print(f"No token logos found under {root / 'blockchains'}", file=sys.stderr)
        return 2

    not_square = 0
    wrong_format = 0
    unreadable = 0
    for path in logos:
        try:
            with Image.open(path) as image:
                size = image.size
                image_format = image.format
                image.verify()
        except (OSError, ValueError, SyntaxError, Image.DecompressionBombError) as error:
            print(f"ERROR\t{path.relative_to(root)}\t{error}")
            unreadable += 1
            continue
        not_square += size[0] != size[1]
        wrong_format += image_format in REJECTED_FORMATS
        if not is_accepted(size, image_format):
            print(f"{size[0]}x{size[1]}\t{image_format}\t{path.relative_to(root)}")

    print(f"Checked {len(logos)} token logos: {not_square} not square, {wrong_format} JPEG, {unreadable} unreadable.")
    return 1 if not_square or wrong_format or unreadable else 0


if __name__ == "__main__":
    sys.exit(main())
