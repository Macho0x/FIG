#include <caml/alloc.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>

#include "fig.h"

CAMLprim value caml_fig_version(value unit) {
    CAMLparam1(unit);
    CAMLreturn(caml_copy_string(fig_version()));
}
