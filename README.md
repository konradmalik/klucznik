[![crates.io](https://img.shields.io/crates/v/klucznik.svg)](https://crates.io/crates/klucznik)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/linting.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/tests.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/nix.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)

# klucznik

Manage your ssh access keys automatically by for ex. synchronizing them from github.

## Installation

```
$ cargo install --locked klucznik
```

## Usage

### Install the binary (optional)

Install the binary to some globally accessible place:

```bash
$ sudo install ~/.cargo/bin/klucznik /usr/local/bin/klucznik
```

### As `authorized_keys` updater

_warning_ this will overwrite your `authorized_keys` file!

Set-up a cron job similar to this:

```bash
* 12 * * * /usr/local/bin/klucznik --source https://github.com/<your username>.keys --destination /home/<user>/.ssh/authorized_keys
```

You can add more sources via more flags.

Alternatively, use [ssh-key-dir](https://github.com/coreos/ssh-key-dir) to not overwrite your `authorized_keys`:

```bash
* 12 * * * /usr/local/bin/klucznik --source https://github.com/<your username>.keys --destination /home/<user>/.ssh/authorized_keys.d/klucznik
```

Then configure your `AuthorizedKeysCommand` in `sshd_config` to use `ssh-key-dir` to that ssh reads your overlays from that folder.

### As `AuthorizedKeysCommand` (experimental!)

Change the following settings in your `sshd_config`:

```
AuthorizedKeysCommand /usr/local/bin/klucznik --source https://github.com/<username>.keys
AuthorizedKeysCommandUser root
```

## Roadmap

### v0.1 'Not much more than overengineered `curl` replacement but works'

- [x] configurable via command-line arguments/flags
- [x] get authorized_keys from public URLs
- [x] validate if in fact keys are returned (basic)
- [x] save to file
- [x] automated cargo release

### v0.1.1

- [ ] if destination is provided, and file-contents are the same, don't overwrite
- [ ] if returned keys are empty, don't overwrite

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
