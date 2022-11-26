[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/linting.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/tests.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/nix.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)

# klucznik

**This program is work-in-progress and for personal purposes. It is nowhere near ready to be used anywhere (but will be :) )**

Manage your ssh access automatically.

## Roadmap

### v1

- [ ] synchronize authorized_keys from public URLs to any local file
- [ ] remove duplicate keys
- [ ] configurable via command-line arguments/flags and ENV variables

### v1.1

- [ ] config file support (remote + local)
- [ ] able to read/store host-specific configuration

### v1.2

- [ ] add authenticated (private) URLs support

### v2

- [ ] add `AuthorizedKeysCommand` support (ability to use this instead of `ssh-key-dir`.

### v2.1

- [ ] ability to authorize directly from URLs (use `AuthorizedKeysCommand` and no `authorized_keys`
- [ ] cache data properly so that it can still authenticate if Github/Gitlab is currently down.
