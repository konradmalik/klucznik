[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/linting.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/tests.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)
[![Actions Status](https://github.com/konradmalik/klucznik/actions/workflows/nix.yml/badge.svg)](https://github.com/konradmalik/klucznik/actions)

# klucznik

Manage your ssh access keys automatically.

## Roadmap

### v0.1 'Not much more than overengineered `curl` replacement but works'

- [x] configurable via command-line arguments/flags
- [x] get authorized_keys from public URLs
- [x] validate if in fact keys are returned (basic)
- [ ] save to file
- [ ] automated cargo release

### v0.2 '`curl` with centralized config'

- [ ] config file support (remote + local)
- [ ] able to read/store host-specific configuration

### v0.3

- [ ] add authenticated (private) URLs support

### v1

- [ ] add `AuthorizedKeysCommand` support (ability to use this instead of `ssh-key-dir`.
- [ ] ability to authorize directly from URLs (use `AuthorizedKeysCommand` and no `authorized_keys`
- [ ] make sure cache data properly so that it can still authenticate if Github/Gitlab is currently down.
