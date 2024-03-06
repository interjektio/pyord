GLOBALPYTHON := python3.10
CARGO := cargo
GIT := git
PYTHON := venv/bin/python
MATURIN := venv/bin/maturin
PYTEST := venv/bin/pytest
RUSTSOURCES := $(shell find src -name '*.rs')


.PHONY: build
build: venv/lib/python3.10/site-packages/pyord/__init__.py


.PHONY: test
test: build
	$(PYTEST)


venv/lib/python3.10/site-packages/pyord/__init__.py: $(RUSTSOURCES) Cargo.toml Cargo.lock | $(PYTHON)
	$(MATURIN) develop
	@# NOTE: maturin includes the .pyi file in the built module, but to generate it, we need to have the module there
	@# in the first place. So we would need another build after this to build the distribution correctly.
	$(PYTHON) generate_stubs.py pyord pyord.pyi --ruff


$(PYTHON):
	$(GLOBALPYTHON) -m venv venv
	$(PYTHON) -m pip install --upgrade pip
	$(PYTHON) -m pip install -e '.[dev]'


submodules/ord/Cargo.toml:
	$(GIT) submodule update --init --recursive
