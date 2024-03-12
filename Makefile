GLOBALPYTHON := python3.10
CARGO := cargo
GIT := git
PYTHON := venv/bin/python
MATURIN := venv/bin/maturin
PYTEST := venv/bin/pytest
RUSTSOURCES := $(shell find src -name '*.rs')
ORDSOURCES := $(shell find submodules/ord -name '*.rs')


.PHONY: develop
develop: venv/lib/python3.10/site-packages/pyord/__init__.py

.PHONY: build
build: develop
	$(MATURIN) build


.PHONY: test
test: develop
	$(PYTEST)


venv/lib/python3.10/site-packages/pyord/__init__.py: $(RUSTSOURCES) $(ORDSOURCES) Cargo.toml Cargo.lock | $(PYTHON)
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


# TODO: figure out a better way. Right now, we will constantly have a dirty (files changed) submodule in the repository.
.PHONY: update-and-patch-ord
update-and-patch-ord:
	git submodule update --init && \
		cd submodules/ord && \
		export COMMIT_HASH=$$(git rev-parse HEAD) && \
		git reset --hard HEAD && \
		git pull && \
		(pushd .. && git add ord && popd) && \
		(git apply ../../ord.patch || (\
			echo "Failed to re-patch ord, restoring to $COMMIT_HASH" && \
			git reset --hard $COMMIT_HASH && \
			exit 1 \
		)) && \
  		echo "Patch successful" && \
		git diff --patch > ../../ord.patch && cd ../../ && git add ord.patch


.PHONY: create-ord-patch
create-ord-patch:
	cd submodules/ord && \
		git diff --patch > ../../ord.patch


.PHONY: patch-ord
patch-ord:
	cd submodules/ord && \
		git reset --hard HEAD && \
		git apply ../../ord.patch


.PHONY: init-submodules
init-submodules:
	git submodule update --init --recursive

.PHONY: build-linux-wheels
build-linux-wheels:
	docker run --rm -it -v $(shell pwd):/io $(shell docker build -q -f Dockerfile.build .) build --release

.PHONY: publish-linux-wheels
publish-linux-wheels:
	docker run --rm -it -v $(shell pwd):/io $(shell docker build -q -f Dockerfile.build .) publish
