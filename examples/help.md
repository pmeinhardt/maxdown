# Help

Print help when invoked with `--help`:

```console
$ maxdown --help
? success
Convert Markdown to HTML

Usage: maxdown [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the input Markdown file [default: stdin]

Options:
  -b, --base <url>       Base URL to use for all relative URLs in the document
      --dangerous        Only use this if you trust the authors of the document
  -o, --output <path>    File to write output to [default: stdout]
  -t, --template <path>  Template to use for output [default: built-in template]
      --title <title>    Title to pass to the template [default: Preview]
  -h, --help             Print help
  -V, --version          Print version

```

Print help when invoked with `-h`:

```console
$ maxdown -h
? success
Convert Markdown to HTML

Usage: maxdown [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the input Markdown file [default: stdin]

Options:
  -b, --base <url>       Base URL to use for all relative URLs in the document
      --dangerous        Only use this if you trust the authors of the document
  -o, --output <path>    File to write output to [default: stdout]
  -t, --template <path>  Template to use for output [default: built-in template]
      --title <title>    Title to pass to the template [default: Preview]
  -h, --help             Print help
  -V, --version          Print version

```
