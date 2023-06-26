all:
	@echo "Moving default config..."
	mkdir -p $(HOME)/.config/rpower
	cp -rn ./config/* $(HOME)/.config/rpower
	@echo "Building..."
	build --release
	@echo "Installing..."
	cargo install --path .
	@echo "Done, enjoy! :)"
