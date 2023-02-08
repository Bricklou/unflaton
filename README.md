# unflaton

Small little program to flatten/unflatten JSON files.

## Compiling

You need to have [Rust](https://www.rust-lang.org/) installed.

```bash
$ cargo build --release
```

## Usage

```bash
$ unflaton -h
Usage: unflaton [OPTIONS] <INPUT> [OUTPUT]

Arguments:
  <INPUT>   The input file to read from
  [OUTPUT]  The output file to write to If not specified, the output will be written to stdout

Options:
  -f, --flatten  Flattent the JSON input This will flatten the keys into a single level of nesting
  -d, --debug    Enable debug logging This will enable debug logging
  -h, --help     Print help
  -V, --version  Print version
```

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details
