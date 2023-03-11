[![crates.io](https://img.shields.io/crates/v/klucznik.svg)](https://crates.io/crates/klucznik)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/main.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/publish.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)

# klucznik

> :warning: **Disclaimer: I use this program to learn Rust, Nix and to automate stuff for personal use.**

Manage your ssh access keys automatically by for ex. synchronizing them from github.

## Installation

```
$ cargo install --locked klucznik
```

## Usage

### Install the binary (optional)

Install the binary to some globally accessible place:

```bash
$ sudo install --mode 755 --owner root --group root ~/.cargo/bin/klucznik /usr/local/bin/klucznik
```

### As `authorized_keys` updater

:warning: this will overwrite your `authorized_keys` file!

Set-up a cron job similar to this:

```bash
* 12 * * * /usr/local/bin/klucznik --sources https://github.com/<your username>.keys --destination /home/<user>/.ssh/authorized_keys
```

You can add more sources via more flags.

Alternatively, use [ssh-key-dir](https://github.com/coreos/ssh-key-dir) to not overwrite your `authorized_keys`:

```bash
* 12 * * * /usr/local/bin/klucznik --sources https://github.com/<your username>.keys --destination /home/<user>/.ssh/authorized_keys.d/klucznik
```

Then configure your `AuthorizedKeysCommand` in `sshd_config` to use `ssh-key-dir` to that ssh reads your overlays from that folder.

### As `AuthorizedKeysCommand` (experimental!)

Change the following settings in your `sshd_config`:

```
AuthorizedKeysCommand /usr/local/bin/klucznik --sources https://github.com/<username>.keys
AuthorizedKeysCommandUser root
```

`AuthorizedKeysCommand` is pretty picky about permissions, ownership etc. of that binary file. Make sure to read the proper `man` entry.

## Roadmap

### v0.1 'Not much more than overengineered `curl` replacement but works'

- [x] configurable via command-line arguments/flags
- [x] get authorized_keys from public URLs
- [x] validate if in fact keys are returned (basic)
- [x] save to file
- [x] automated cargo release

### v0.1.1

- [x] fix writing multiple sources to one file
- [x] if destination is provided, and file-contents are the same, don't overwrite
- [x] if returned keys are empty, don't overwrite

### v0.1.2

- [x] fix created file permissions to 600

### v0.2 '`curl` with centralized config'

- [ ] deduplicate keys
- [ ] config file support (remote + local)
- [ ] able to read/store host-specific configuration

### v0.3

- [ ] add authenticated (private) URLs support

### v1

- [ ] refine `AuthorizedKeysCommand` support (ability to use this instead of `ssh-key-dir`.
- [ ] ability to authorize directly from URLs (use `AuthorizedKeysCommand` and no `authorized_keys`
- [ ] make sure cache data properly so that it can still authenticate if Github/Gitlab is currently down.
- [ ] ability to chain commands in `AuthorizedKeysCommand`
