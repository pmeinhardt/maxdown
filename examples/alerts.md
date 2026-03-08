# Render GitHub-style alerts

Convert Markdown with alert block-quotes:

```console
$ maxdown alerts.md
? success
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>Preview</title>
    <base href="">
  </head>
  <body>
    <div class="markdown-alert markdown-alert-note">
<p class="markdown-alert-title">Note</p>
<p>Highlights information that users should take into account, even when skimming.</p>
</div>
<div class="markdown-alert markdown-alert-tip">
<p class="markdown-alert-title">Tip</p>
<p>Optional information to help a user be more successful.</p>
</div>
<div class="markdown-alert markdown-alert-important">
<p class="markdown-alert-title">Important</p>
<p>Crucial information necessary for users to succeed.</p>
</div>
<div class="markdown-alert markdown-alert-warning">
<p class="markdown-alert-title">Warning</p>
<p>Critical content demanding immediate user attention due to potential risks.</p>
</div>
<div class="markdown-alert markdown-alert-caution">
<p class="markdown-alert-title">Caution</p>
<p>Negative potential consequences of an action.</p>
</div>
  </body>
</html>

```
