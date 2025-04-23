include config.mk

all: build

build:
ifeq ($(ENABLE_ERROR_HANDLING),1)
	cargo build --release
else
	cargo build --release --no-default-features
endif

clean:
	rm -f config.mk
	cargo clean

install:
	install -Dm755 target/release/catfetch -t $(DESTDIR)$(BINDIR)/

uninstall:
	rm -f  $(DESTDIR)$(BINDIR)/catfetch
