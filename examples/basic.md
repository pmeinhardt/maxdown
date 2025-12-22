# Convert Markdown to HTML

Convert Markdown:

```console
$ maxdown input.md
? success
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Preview</title>
    <base href="">
  </head>
  <body>
    <p>Hello <strong>Markdown</strong>!</p>
  </body>
</html>

```

Convert Markdown with custom document title:

```console
$ maxdown --title "Pizzazz" input.md
? success
<!DOCTYPE html>
<html>
  <head>
...
    <title>Pizzazz</title>
...
  </head>
  <body>
    <p>Hello <strong>Markdown</strong>!</p>
  </body>
</html>

```

Convert Markdown with custom base URL:

```console
$ maxdown --base "https://github.com/pmeinhardt/maxdown" input.md
? success
<!DOCTYPE html>
<html>
  <head>
...
    <base href="https://github.com/pmeinhardt/maxdown">
...
  </head>
  <body>
    <p>Hello <strong>Markdown</strong>!</p>
  </body>
</html>

```

Convert Markdown with custom template:

```console
$ maxdown --template template.html input.md
? success
<!DOCTYPE html>
<html>
  <head><title>Fancy Template</title></head>
  <body><p>Hello <strong>Markdown</strong>!</p></body>
</html>

```
