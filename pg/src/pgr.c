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
    PgMemoryChunk chunk;

    chunk = hello_world();
    if (chunk.error > 0)
    {
        PG_RETURN_TEXT_P(cstring_to_text("Error"));
    }

    PG_RETURN_TEXT_P(cstring_to_text(chunk.ptr));
}
