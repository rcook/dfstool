from argparse import ArgumentParser
from collections.abc import Generator, Sequence
from pathlib import Path
from typing import TextIO, TypeVar
import sys

T = TypeVar("T")


def chunks(items: Sequence[T], n: int) -> Generator[list[T], None, None]:
    for i in range(0, len(items), n):
        yield items[i:i + n]


def generate_rust(input_path: Path, output_path: Path | None) -> None:
    def open_output() -> TextIO:
        if output_path is None:
            return sys.stdout
        else:
            return output_path.open("wt")

    with input_path.open("rb") as f:
        bytes = f.read()

    with open_output() as f:
        print(f"const BYTES: [u8; {len(bytes)}] = [")
        for chunk in chunks(bytes, 16):
            s = " ".join(f"0x{b:02x}," for b in chunk)
            print("    "+s, file=f)
        print("];")


def main(cwd: Path, argv: list[str]) -> None:
    def resolved_path(s: str) -> Path:
        return (cwd / Path(s).expanduser()).resolve()

    parser = ArgumentParser(prog="bin2rs")
    _ = parser.add_argument(
        "input_path",
        metavar="INPUT_PATH",
        type=resolved_path)
    _ = parser.add_argument(
        "-o",
        dest="output_path",
        metavar="OUTPUT_PATH",
        type=resolved_path,
        required=False)
    args = parser.parse_args(argv)
    generate_rust(args.input_path, args.output_path)


if __name__ == "__main__":
    main(cwd=Path.cwd(), argv=sys.argv[1:])
