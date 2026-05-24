BINDIR ?= $(HOME)/.local/bin
ZIMDIR ?= $(HOME)/.zim/modules/completion/functions
BINARY = proj-core
TARGET = target/release/$(BINARY)

.PHONY: build install uninstall clean zim-install

build: $(TARGET)

$(TARGET): src/main.rs Cargo.toml
	cargo build --release

install: $(TARGET)
	mkdir -p $(BINDIR)
	cp $(TARGET) $(BINDIR)/$(BINARY)
	strip $(BINDIR)/$(BINARY)
	@echo "Installed $(BINDIR)/$(BINARY)"
	@echo ""
	@echo "Add to .zshrc:"
	@echo "  eval \"\$$($(BINARY) shell func)\""
	@echo "  eval \"\$$($(BINARY) shell completion --shell zsh)\""


zim-install:
	mkdir -p $(ZIMDIR)
	$(BINDIR)/$(BINARY) completion > $(ZIMDIR)/_proj
	@echo "Installed completion to $(ZIMDIR)/_proj"
	@echo "Restart shell or run: exec zsh"

uninstall:
	rm -f $(BINDIR)/$(BINARY)
	rm -f $(ZIMDIR)/_proj
	@echo "Removed $(BINDIR)/$(BINARY), $(ZIMDIR)/_proj"

clean:
	cargo clean
