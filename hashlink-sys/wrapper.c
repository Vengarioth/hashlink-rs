// -D
#define UNICODE
#define _UNICODE
#define HL_VERSION_MAJOR 1
#define HL_VERSION_MINOR 11
#define HL_VERSION_PATCH 0
#define HL_VERSION HL_VERSION_MAJOR.HL_VERSION_MINOR.HL_VERSION_PATCH
#define LIBHL_EXPORTS

#include "../vendor/hashlink/src/hl.h"
#include "../vendor/hashlink/src/hlc.h"
#include "../vendor/hashlink/src/hlmodule.h"

#include "../vendor/hashlink/include/pcre/pcre_chartables.c"
#include "../vendor/hashlink/include/pcre/pcre_compile.c"
#include "../vendor/hashlink/include/pcre/pcre_dfa_exec.c"
#include "../vendor/hashlink/include/pcre/pcre_exec.c"
#include "../vendor/hashlink/include/pcre/pcre_fullinfo.c"
#include "../vendor/hashlink/include/pcre/pcre_globals.c"
#include "../vendor/hashlink/include/pcre/pcre_newline.c"
#include "../vendor/hashlink/include/pcre/pcre_string_utils.c"
#include "../vendor/hashlink/include/pcre/pcre_tables.c"
#include "../vendor/hashlink/include/pcre/pcre_ucd.c"
#include "../vendor/hashlink/include/pcre/pcre_xclass.c"
#include "../vendor/hashlink/include/pcre/pcre16_ord2utf16.c"
#include "../vendor/hashlink/include/pcre/pcre16_valid_utf16.c"

#include "../vendor/hashlink/src/gc.c"

#include "../vendor/hashlink/src/std/array.c"
#include "../vendor/hashlink/src/std/buffer.c"
#include "../vendor/hashlink/src/std/bytes.c"
#include "../vendor/hashlink/src/std/cast.c"
#include "../vendor/hashlink/src/std/date.c"
#include "../vendor/hashlink/src/std/error.c"
#include "../vendor/hashlink/src/std/file.c"
#include "../vendor/hashlink/src/std/fun.c"
#include "../vendor/hashlink/src/std/maps.c"
#include "../vendor/hashlink/src/std/math.c"
#include "../vendor/hashlink/src/std/obj.c"
#include "../vendor/hashlink/src/std/random.c"
#include "../vendor/hashlink/src/std/regexp.c"
#include "../vendor/hashlink/src/std/socket.c"
#include "../vendor/hashlink/src/std/string.c"
#include "../vendor/hashlink/src/std/sys.c"
#include "../vendor/hashlink/src/std/track.c"
#include "../vendor/hashlink/src/std/types.c"
#include "../vendor/hashlink/src/std/ucs2.c"
#include "../vendor/hashlink/src/std/thread.c"
#include "../vendor/hashlink/src/std/process.c"

#include "../vendor/hashlink/src/code.c"
#include "../vendor/hashlink/src/jit.c"
#include "../vendor/hashlink/src/module.c"
#include "../vendor/hashlink/src/debugger.c"
#include "../vendor/hashlink/src/profile.c"
