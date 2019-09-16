# CargoMake by NeoSmart Technologies
# Written and maintained by Mahmoud Al-Qudsi <mqudsi@neosmart.net>
# Released under the MIT public license
# Obtain updates from https://github.com/neosmart/CargoMake

prefix = /usr
libprefix = ${prefix}/lib
DESTDIR=
SHARED_OBJECT = target/release/libnss_zerotier.so
INSTALL_NAME = libnss_zerotier.so.2

COLOR ?= always # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)

.PHONY: all build-release clean install

all: clean build-release

clean:
	@$(CARGO) clean

build-release:
	@$(CARGO) build --release

install:
	install -m755 -d $(DESTDIR)$(libprefix)/
	install -m644 $(SHARED_OBJECT) $(DESTDIR)$(libprefix)/$(INSTALL_NAME)
	ldconfig -n $(DESTDIR)$(libprefix)/
