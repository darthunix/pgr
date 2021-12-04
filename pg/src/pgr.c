#include <postgres.h>
#include <fmgr.h>
#include <miscadmin.h>
#include <utils/builtins.h>

#include "rust4pg.h"

#ifdef PG_MODULE_MAGIC
PG_MODULE_MAGIC;
#endif

PG_FUNCTION_INFO_V1(hello_rust);
Datum
hello_rust(PG_FUNCTION_ARGS) {
    char *buf;
    int len = 100;

    buf = palloc(sizeof(char) * len);
    if (hello_world(buf, len) > 0) {
        PG_RETURN_TEXT_P(cstring_to_text(buf));
    }

    PG_RETURN_TEXT_P(cstring_to_text("ERROR"));
}
