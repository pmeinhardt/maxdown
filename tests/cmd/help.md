# Test: help

Print help when invoked without arguments:

```console
$ maxdown
? 2
Convert Markdown to HTML

Usage: maxdown [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the input markdown file or "-" for stdin

Options:
  -b, --base <url>       Base URL to use for all relative URLs in the document
      --dangerous        Only use this if you trust the authors of the document
  -o, --output <path>    File to write output to [default: stdout]
  -t, --template <path>  Template to use for output
      --title <title>    Title to pass to the template [default: Preview]
  -h, --help             Print help
  -V, --version          Print version

```

Print help when invoked with `--help`:

```console
$ maxdown --help
Convert Markdown to HTML

Usage: maxdown [OPTIONS] <PATH>

Arguments:
  <PATH>  Path to the input markdown file or "-" for stdin

Options:
  -b, --base <url>       Base URL to use for all relative URLs in the document
      --dangerous        Only use this if you trust the authors of the document
  -o, --output <path>    File to write output to [default: stdout]
  -t, --template <path>  Template to use for output
      --title <title>    Title to pass to the template [default: Preview]
  -h, --help             Print help
  -V, --version          Print version

```
