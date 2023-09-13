.PHONY: default doc clean

default:
	@echo 'Usage:'
	@echo '	make doc: Generate API documents'

doc:
	cargo doc --package newslab-serde --all-features --release --open

clean:
	cargo clean
