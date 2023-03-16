PREFIX ?= /usr/bin
CARGO ?= cargo

all:
	@echo "Moving default config..."
	@mkdir -p $(HOME)/.config/rpower
	@cp -rn ./config/* $(HOME)/.config/rpower
	@echo "Building..."
	@$(CARGO) build --release
	@echo "Done!"

install:
	@echo "Installing..."
	@install -Dm755 ./target/release/rpower $(PREFIX)/rpower
	@echo "Done, enjoy! :)"
