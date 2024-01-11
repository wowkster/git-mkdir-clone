# git-mkdir-clone

A simple CLI tool for automatically creating the parent directory when cloning a git repository.

## Usage

```console
$ git-mkdir-clone https://github.com/wowkster/git-mkdir-clone.git
```

This will create the `wowkster` directory if it does not already exist, and then clone the `git-mkdir-clone` repository inside that folder.

## Alias

You might find it more convenient to alias the tool to a shorter name in your shell configuration file. Personally I use the following alias:

```sh
alias gmc="git-mkdir-clone"
```