NAME :=		cbacklight
RELEASE_TYPE :=	release
BUILDDIR :=	target
TARGET :=	$(BUILDDIR)/$(RELEASE_TYPE)/$(NAME)
DESTDIR :=	/usr/local
BINDIR :=	$(DESTDIR)/bin
MANPATH :=	$(DESTDIR)/man
SOURCES :=	src/main.rs
INSTALLABLE :=	$(BINDIR)/$(NAME) $(MANPATH)/man1/$(NAME).1

.PHONY: all
all: $(TARGET)

$(TARGET): $(SOURCES)
ifeq ($(RELEASE_TYPE), debug)
	cargo build
else ifeq ($(RELEASE_TYPE), release)
	cargo build --release
else
	echo "Invalid release type '$(RELEASE_TYPE)'" > /dev/stderr
	exit 1
endif

.PHONY:
install: $(INSTALLABLE)

$(BINDIR)/$(NAME): $(TARGET)
	install -o root -g root -m 0755 -D $(TARGET) $(BINDIR)/$(NAME)

$(MANPATH)/man1/$(NAME).1: $(NAME).1
	install -o root -g root -m 0755 -D $(TARGET) $(BINDIR)/$(NAME)

uninstall:
	rm -f $(INSTALLABLE)

.PHONY: clean
clean:
	cargo clean
	cargo clean --release

