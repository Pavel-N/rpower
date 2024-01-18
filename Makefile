all:
	@echo "Moving default config..."
	mkdir -p $$HOME/.config/rpower
	cp -r --update=none ./config/* $$HOME/.config/rpower
	@echo "Building..."
	cargo build --release
	@echo "Installing..."
	cargo install --path .
	@echo "Done, enjoy! :)"
