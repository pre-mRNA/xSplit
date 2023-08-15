# xSplit

**xSplit** is a versatile and high-performance command-line tool that efficiently splits large tabular data files into multiple segments. By targeting a specific column, users can create uniform splits that keep sequential rows with matching values in the same file. Designed with bioinformatics applications in mind, xSplit handles massive datasets with ease and precision.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Command-line Options](#command-line-options)
- [Examples](#examples)
- [Building from Source](#building-from-source)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)

## Features

- **Dynamic Splitting**: Customize the number of output files and designate the common column index for controlled splitting.
- **Efficient Processing**: Optimized for speed and low memory consumption, perfect for large-scale data processing.
- **Cross-Platform Support**: Binaries available for Windows, Linux, and macOS.
- **Custom Delimiters**: Works seamlessly with various delimiters, including tabs, commas, or any user-defined character.
- **Header Preservation**: Retains the header row across all output files for consistent data structure.
- **Error Handling**: Intuitive error messages and warnings to guide users.

## Installation

Pre-compiled binaries are available for various platforms on the [Releases](URL_TO_RELEASES_PAGE) page. Simply download the binary suitable for your operating system and place it in a directory included in your system's PATH.

## Usage

Basic command structure:

```bash
xSplit --splits NUMBER --prefix PREFIX --column INDEX [OPTIONS]
```

### Command-line Options

- `-n`, `--splits NUMBER`: Number of output files to create (required).
- `-p`, `--prefix PREFIX`: Prefix for naming the output files (required).
- `-c`, `--column INDEX`: Common column index (1-based) to guide the split (required).
- `-d`, `--delimiter DELIMITER`: Delimiter for splitting columns (default: tab).

## Examples

Split a file into 4 parts based on the 3rd column:

```bash
cat input.tsv | xSplit --splits 4 --prefix ./output --column 3
```

Using a comma as a delimiter:

```bash
xSplit --splits 5 --prefix ./output --column 2 --delimiter ","
```

## Building from Source

If you prefer to build from source, you'll need Rust and Cargo installed:

1. Clone the repository: `git clone URL_TO_REPOSITORY`
2. Navigate to the project directory: `cd xSplit`
3. Build the project: `cargo build --release`
4. The binary will be available in `target/release`

## License

This project is licensed under the MIT License. See the [LICENSE.md](URL_TO_LICENSE_FILE) file for complete details.

## Contact

- **Author**: AJ Sethi
- **Email**: aditya.sethi@anu.edu.au

For support, questions, or collaboration, feel free to reach out.

