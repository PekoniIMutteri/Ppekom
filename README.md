# Ppekom

(pronounced pepekom)
This library will read or write Pimages from PNM P6 (binary PPM) files.

## Usage

There are only 2 functions, one to read a file, the other to write to a file.

## Download

I haven't tried putting it on cargo (and don't even know if that's possible) so for
now, you have to download the repository, and add it to your dependencies like so:

```toml
[dependencies]
ppekom = {path = "your_path_to_ppekom"}
```

This library expects the Pimage repository to be on the same level as it, like this:

- WhateverFolder
  - Pimage
  - Ppekom

## ROADMAP

This library already helped find many bugs (3, to be exact) in my Pimage library,
but as always, bugfixes.

For now, I have no plan to add that, but I might someday make it compatible with
P1 to P6 files.
