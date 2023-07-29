# mdBook Nix preprocessor

mdBook preprocessor to render [Nix](https://nixos.org/nix) files through [Nixdoc](https://github.com/nix-community/nixdoc/).

## Contents
- [Usage](#usage)

## Usage

First create the preprocessor in your book.toml file:
```toml
[preprocessor.nixdoc]
command = "mdbook-nixdoc"
```

The above assumes both mdbook-nixdoc and nixdoc executables are on your path.

Then add a Nix file to your `SUMMARY.md`:
```markdown
# Contents

- [Getting started](./getting-started.md)
- [Hacking](./HACKING.md)
- [FAQ](./FAQ.md)

# Reference

- [User facing APIs](./reference.md)
  - [project](./lib/project.nix)
```
