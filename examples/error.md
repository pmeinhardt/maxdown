# Error output

Print error when input file cannot be read:

```console
$ maxdown enoent.md
? failed
Error: Failed to read input from enoent.md

Caused by:
    No such file or directory (os error 2)

```

Print error when template file cannot be read:

```console
$ maxdown --template enoent.html input.md
? failed
Error: Failed to read template from enoent.html

Caused by:
    No such file or directory (os error 2)

```

Print error when output file cannot be written:

```console
$ maxdown --output enoent/output.html input.md
? failed
Error: Failed to write output to enoent/output.html

Caused by:
    No such file or directory (os error 2)

```
