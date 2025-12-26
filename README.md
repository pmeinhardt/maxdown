# Maxdown 🚲

[![checks](https://github.com/pmeinhardt/maxdown/actions/workflows/build.yml/badge.svg)](https://github.com/pmeinhardt/maxdown/actions/workflows/build.yml)

Maxdown is a simple and fast *Markdown-to-HTML converter*.

It comes with a command-line interface and an integration for Vim and Neovim.

![](./media/banner.png)

## Before you get going 🐿️

You will need `rustc` and `cargo` to build the project. It is recommended to install the Rust toolchain via [`rustup`](https://rust-lang.org/tools/install/).

In case you are using [Homebrew](https://brew.sh/), you can use the `rustup` formula.

```shell
# Install Rustup via Homebrew
brew install rustup

# Set default toolchain to stable and install it
rustup default stable
```

## Installation 🪛

### Command-line tool

If you're only interested in the `maxdown` command-line tool, you can build it by running:

```shell
cargo build --release --locked
```

Then, just put the resulting `target/release/maxdown` binary somewhere on your `PATH`.

### Command-line tool via Homebrew

Alternatively, you can install `maxdown`, built from source, via Homebrew:

```shell
brew install --HEAD pmeinhardt/tools/maxdown
```

To update, use:

```shell
brew upgrade --fetch-HEAD pmeinhardt/tools/maxdown
```

### Vim plugin

For the Vim plugin, if you use [vim-plug](https://github.com/junegunn/vim-plug), add this line to your Vim configuration file:

```vim
Plug 'pmeinhardt/maxdown', {'do': ':MaxdownCompile'}
```

If you have a different way of managing your Vim plugins, make sure you add the project directory to your `runtimepath` and build the command-line tool:

```shell
cargo build --release --locked
```

Or, to build from within Vim:

```
:MaxdownCompile
```

## Usage ⌨️

### Command-line tool

The `maxdown` command-line tool reads Markdown from `stdin` or a file and outputs HTML to `stdout` or a file.

For instance:

```shell
echo 'Hello *World*!' | maxdown
maxdown README.md
```

To learn about the extra knobs and switches, kindly ask `maxdown` for help:

```
$ maxdown --help
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

### Vim plugin

To preview the current Markdown buffer, invoke `:MaxdownPreview`.

To replace the buffer's content with the corresponding HTML, use `:MaxdownConvert`.

If you use these more frequently, define a custom mapping. For instance:

```vim
" Create a normal-mode mapping for previewing Markdown. Adjust the key sequence as you like.
autocmd FileType markdown nnoremap <buffer> <localleader>m <Plug>MaxdownPreview
```

If you have other use cases, you always have the option of defining your own custom commands and mappings using the command-line tool.

## Enhancements 🪄

When paired with the `ql` [Vim plugin](https://github.com/pmeinhardt/ql), Maxdown previews on macOS will look more neat (no `[DEBUG]` in the title):

```vim
Plug 'pmeinhardt/ql', {'do': ':QuickLookCompile'}
```

## Thanks ❤️

Maxdown is built on top of the impressive work of other people:

- markdown-rs: https://github.com/wooorm/markdown-rs
- minijinja: https://github.com/mitsuhiko/minijinja
- clap: https://github.com/clap-rs/clap
- trycmd: https://github.com/assert-rs/trycmd
- github-markdown-css: https://github.com/sindresorhus/github-markdown-css
