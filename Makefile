.PHONY: build
build:
	@cargo build


.PHONY: format
format:
	@cargo fmt


.PHONY: check
check:
	@cargo check


.PHONY: shortcheck
shortcheck:
	@cargo check --message-format short


.PHONY: test
test:
	@cargo test


.PHONY: clippy
clippy:
	@cargo clippy
