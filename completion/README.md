# Completions

_orthanc-cli_ provides pre-build completion files for Bash, fish and Zsh. The completion files are located in
[completion](../completion) directory. This directory is also bundled with each release tarball of _orthanc-cli_.

## Installation

### Bash

First make sure `bash-completion` package is installed on your system. Refer to the official
[documentation](https://github.com/scop/bash-completion#installation) for installation details. Then put the file
`orthanc.bash` into `~/.local/share/bash-completion/completions/` directory.

### fish

Put the file `orthanc.fish` into `~/.config/fish/completions` directory.

### Zsh

Put the file `_orthanc` into a directory of your liking (e.g. `~/.zsh_completions`), and add this directory to your
`fpath`: `fpath = ($fpath ~/.zsh_completions)`.
