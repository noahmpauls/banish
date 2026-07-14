# Banish

A tiny command line utility for quickly blocking a domain via the /etc/hosts file.

## Install

1. Clone this repo.
2. In the repo root, run `cargo install --path .`

## Usage:

Since the tool writes to the `/etc/hosts` file, you must run `banish` with admin privileges:

```
$ sudo banish https://www.youtube.com
```
