EXTENSION = pgr
MODULES = pgr
DATA = pgr--0.1.sql
MODULE_big = pgr

OBJS = \
	src/pgr.o \

EXTRA_CLEAN = \
	src/rust4pg.h \
	rust_build \

PG_CFLAGS = -I$(CURDIR)/src -L$(CURDIR)/rust_build/debug
SHLIB_LINK += -lrust4pg

PG_CONFIG = pg_config
PGXS := $(shell $(PG_CONFIG) --pgxs)
include $(PGXS)

src/pgr.o: src/rust4pg.h
src/pgr.o: rust4pg.so
src/rust4pg.h:
	cbindgen --crate rust4pg --lang c --output src/rust4pg.h ../rust
rust4pg.so:
	cargo build --lib --target-dir rust_build --manifest-path ../rust/Cargo.toml
