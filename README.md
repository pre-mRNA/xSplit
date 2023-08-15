# xSplit

**xSplit** is a versatile command-line tool engineered for rapid and controlled splitting of large tabular data files into multiple segments. By targeting a specific column, it groups adjacent rows with identical values, facilitating precise segmentation. Designed with a minimal memory footprint, xSplit is ideal for bioinformatics, big data applications, and parallel processing of extensive datasets.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Building from Source](#building-from-source)
- [Contact](#contact)

## Features

- **Dynamic Splitting**: Segment files based on a target column, ensuring uniform distribution.
- **High Efficiency**: Optimized for speed with minimal memory usage, tailored for large-scale data.
- **Flexible Delimiters**: Supports various delimiters, including tabs, commas, or user-defined characters.
- **Cross-Platform**: Pre-compiled binaries available for Linux and macOS.

## Installation

Choose the appropriate binary for your OS from the [Releases](https://github.com/pre-mRNA/xSplit/releases/tag/v1.0.0) page and add it to your PATH. Alternatively, build from source (instructions below).

## Usage

Invoke xSplit with:

```bash
xSplit --splits NUMBER --prefix PREFIX --column INDEX [OPTIONS]
```

Options:

- `-n`, `--splits`: Define output files (required).
- `-p`, `--prefix`: Name output files (required).
- `-c`, `--column`: Select common column index (1-based, required).
- `-d`, `--delimiter`: Specify delimiter (default: tab).

Example:

Split a file by the 3rd column into 4 segments:

```bash
cat input.tsv | xSplit -n 4 -p ./output -c 3
```

## Building from Source

1. Clone the repository: `git clone URL_TO_REPOSITORY`
2. Navigate to the directory: `cd xSplit`
3. Compile: `cargo build --release`
4. Find the binary in `target/release`

## Contact

- **Author**: AJ Sethi
- **Email**: aditya.sethi@anu.edu.au
- **License**: [MIT](https://github.com/pre-mRNA/xSplit/blob/main/LICENSE)

_xSplit offers a specialized solution for segmenting large tabular data, maintaining the integrity of related rows. Its efficiency and adaptability make it a valuable tool for researchers, data scientists, and professionals handling vast datasets._

