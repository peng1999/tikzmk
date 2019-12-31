# tikzmk

## Installation

```console
$ cargo install --git https://github.com/peng1999/tikzmk.git
```

## Usage

This tool is intended to work with Ti*k*Z snippet that can be `\input`ed in a LaTeX file.
You can use your favorite editor, for example Vim, to edit `tikz` file, and set `'makeprg'` option:

```vim
set makeprg=tikzmk\ %\ -x
```

And then `:mak --open` to compile and open a default PDF viewer showing the produced picture. Every time a change is made, a `:mak` command will update the picture.

If any Ti*k*Z library or LaTeX package should be used, declare it at the begining of the Ti*k*Z file:

```tex
%% tikzlibrary:calc,positioning package:tikzcd
%% package:ctex
```

Note that the space after `%%` is mandatory.
