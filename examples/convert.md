# Convert Markdown to HTML

Convert Markdown:

```console
$ maxdown examples/fixture.md
? success
<!DOCTYPE html>
<html>
  <head>
...
    <title>Preview</title>
    <base href="">
...
  </head>
  <body class="markdown-body">
    <p>Hello <strong>Markdown</strong>!</p>

  </body>
</html>


```

Convert Markdown with custom document title:

```console
$ maxdown --title "Pizzazz" examples/fixture.md
? success
<!DOCTYPE html>
<html>
  <head>
...
    <title>Pizzazz</title>
...
  </head>
  <body class="markdown-body">
    <p>Hello <strong>Markdown</strong>!</p>

  </body>
</html>


```

Convert Markdown with custom base URL:

```console
$ maxdown --base "https://github.com" examples/fixture.md
? success
<!DOCTYPE html>
<html>
  <head>
...
    <base href="https://github.com">
...
  </head>
  <body class="markdown-body">
    <p>Hello <strong>Markdown</strong>!</p>

  </body>
</html>


```
