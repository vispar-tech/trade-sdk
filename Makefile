.PHONY: setup-pre-commit install-pre-commit test-pre-commit clean-venv

# Setup Python virtual environment and install pre-commit
setup-pre-commit:
	python3 -m venv .venv
	source .venv/bin/activate && pip install pre-commit
	source .venv/bin/activate && pre-commit install

# Install pre-commit hooks (assumes .venv exists)
install-pre-commit:
	source .venv/bin/activate && pre-commit install

# Test pre-commit on all files
test-pre-commit:
	source .venv/bin/activate && pre-commit run --all-files

# Remove virtual environment
clean-venv:
	rm -rf .venv

# Full setup (Python env + pre-commit)
setup-all: setup-pre-commit