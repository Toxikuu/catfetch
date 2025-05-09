include config.mk

all: build

build:
	cargo build --release

clean:
	rm -f config.mk
	cargo clean

install:
	install -Dm755 target/release/catfetch -t $(DESTDIR)$(BINDIR)/

uninstall:
	rm -f  $(DESTDIR)$(BINDIR)/catfetch
