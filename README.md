Python wrappers for Ordinals
============================

This project provides Python wrappers for [ord](https://github.com/ordinals/ord) internals.

The project is very much WIP. Currently, only wrappers for structs and functions related to Runes are provided.

The philosophy is to wrap `ord` internal structs as thinly as possible inside pyo3-compatible Rust, and to
provide sane methods on top of them to enable use in Python.

At the time of writing, patching `ord` is necessary to expose certain structs and functions that are not exposed
in the public api of `ord`. Make targets `patch-ord` and `update-and-patch-ord` are provided for this.

## Development

```bash
# python3.10 needs to be in PATH
make init-submodules
make patch-ord
make develop  # creates a venv and installs `pyord` inside it
make test  # test using pytest
```

## Building wheels

```bash
make build
```
