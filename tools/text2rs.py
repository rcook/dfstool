from argparse import ArgumentParser
from pathlib import Path
from typing import TextIO
import sys


def generate_rust(input_path: Path, output_path: Path | None) -> None:
    def open_output() -> TextIO:
        if output_path is None:
            return sys.stdout
        else:
            return output_path.open("wt")

    with input_path.open("rb") as f:
        bytes = f.read()

    s = ""
    for b in bytes:
        if b < 32 or b > 127:
            s += f"\\u{{{b:02X}}}"
        else:
            c = chr(b)
            if c == "\"":
                s += "\\\""
            else:
                s += c

    with open_output() as f:
        print(f"\"{s}\"", file=f)


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
