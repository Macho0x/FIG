#include <caml/alloc.h>
#include <caml/custom.h>
#include <caml/fail.h>
#include <caml/memory.h>
#include <caml/mlvalues.h>

#include "fig.h"

#define Client_val(v) (*((struct FigClientHandle **)Data_custom_val(v)))
#define Sub_val(v) (*((struct FigSubHandle **)Data_custom_val(v)))

static void fig_client_finalize(value v) {
    struct FigClientHandle *h = Client_val(v);
    if (h != NULL) {
        fig_client_close(h);
        Client_val(v) = NULL;
    }
}

static void fig_sub_finalize(value v) {
    struct FigSubHandle *h = Sub_val(v);
    if (h != NULL) {
        fig_client_sub_close(h);
        Sub_val(v) = NULL;
    }
}

static struct custom_operations fig_client_ops = {
    "fig.FigClient",
    fig_client_finalize,
    custom_compare_default,
    custom_hash_default,
    custom_serialize_default,
    custom_deserialize_default,
    custom_compare_ext_default,
    NULL,
};

static struct custom_operations fig_sub_ops = {
    "fig.FigSub",
    fig_sub_finalize,
    custom_compare_default,
    custom_hash_default,
    custom_serialize_default,
    custom_deserialize_default,
    custom_compare_ext_default,
    NULL,
};

static value alloc_ptr(void *p, struct custom_operations *ops) {
    value v = caml_alloc_custom(ops, sizeof(void *), 0, 1);
    *((void **)Data_custom_val(v)) = p;
    return v;
}

static value bytes_of_fig_buffer(struct FigBuffer buf) {
    value v;
    if (buf.data == NULL || buf.len == 0) {
        fig_buffer_free(buf);
        return caml_alloc_string(0);
    }
    v = caml_alloc_initialized_string(buf.len, (const char *)buf.data);
    fig_buffer_free(buf);
    return v;
}

static value bytes_of_frame_list(struct FigFrameList list) {
    CAMLparam0();
    CAMLlocal2(arr, s);
    mlsize_t n = (list.frames != NULL && list.count > 0) ? (mlsize_t)list.count : 0;
    arr = caml_alloc(n, 0);
    for (mlsize_t i = 0; i < n; i++) {
        struct FigBuffer buf = list.frames[i];
        if (buf.data == NULL || buf.len == 0) {
            s = caml_alloc_string(0);
        } else {
            s = caml_alloc_initialized_string(buf.len, (const char *)buf.data);
        }
        Store_field(arr, i, s);
    }
    fig_frame_list_free(list);
    CAMLreturn(arr);
}

CAMLprim value caml_fig_version(value unit) {
    CAMLparam1(unit);
    CAMLreturn(caml_copy_string(fig_version()));
}

CAMLprim value caml_fig_sbe_encode_new_order_single(
    value cl, value symbol, value side, value qty, value price) {
    CAMLparam5(cl, symbol, side, qty, price);
    struct FigBuffer out = {0};
    int32_t rc = fig_sbe_encode_new_order_single(
        String_val(cl),
        String_val(symbol),
        Bool_val(side) ? 1 : 0,
        Double_val(qty),
        Double_val(price),
        -1,
        -1,
        &out);
    if (rc != 0) {
        caml_failwith("fig_sbe_encode_new_order_single failed");
    }
    CAMLreturn(bytes_of_fig_buffer(out));
}

CAMLprim value caml_fig_jwt_encode(value sub, value exp, value secret) {
    CAMLparam3(sub, exp, secret);
    struct FigBuffer out = {0};
    int32_t rc = fig_jwt_encode(
        String_val(sub), (uint64_t)Int64_val(exp), String_val(secret), &out);
    if (rc != 0) {
        caml_failwith("fig_jwt_encode failed");
    }
    CAMLreturn(bytes_of_fig_buffer(out));
}

CAMLprim value caml_fig_jwt_verify_bearer(value token, value secret) {
    CAMLparam2(token, secret);
    if (fig_jwt_verify_bearer(String_val(token), String_val(secret)) != 0) {
        caml_failwith("fig_jwt_verify_bearer failed");
    }
    CAMLreturn(Val_unit);
}

CAMLprim value caml_fig_frame_encode_subscribe_auth(
    value channel_id, value stream_seq, value routing, value path, value auth) {
    CAMLparam5(channel_id, stream_seq, routing, path, auth);
    struct FigBuffer out = {0};
    const char *tok = caml_string_length(auth) ? String_val(auth) : NULL;
    int32_t rc = fig_frame_encode_subscribe_auth(
        (uint16_t)Int_val(channel_id),
        (uint32_t)Int_val(stream_seq),
        String_val(routing),
        String_val(path),
        tok,
        &out);
    if (rc != 0) {
        caml_failwith("fig_frame_encode_subscribe_auth failed");
    }
    CAMLreturn(bytes_of_fig_buffer(out));
}

CAMLprim value caml_fig_client_connect(value addr, value server_name) {
    CAMLparam2(addr, server_name);
    CAMLlocal1(v);
    struct FigClientHandle *out = NULL;
    const char *name =
        caml_string_length(server_name) ? String_val(server_name) : NULL;
    if (fig_client_connect(String_val(addr), name, &out) != 0) {
        caml_failwith("fig_client_connect failed");
    }
    v = alloc_ptr(out, &fig_client_ops);
    CAMLreturn(v);
}

CAMLprim value caml_fig_client_close(value client) {
    CAMLparam1(client);
    struct FigClientHandle *h = Client_val(client);
    if (h != NULL) {
        fig_client_close(h);
        Client_val(client) = NULL;
    }
    CAMLreturn(Val_unit);
}

CAMLprim value caml_fig_client_ping(value client) {
    CAMLparam1(client);
    if (fig_client_ping(Client_val(client)) != 0) {
        caml_failwith("fig_client_ping failed");
    }
    CAMLreturn(Val_unit);
}

CAMLprim value caml_fig_client_request_and_recv(value client, value frame) {
    CAMLparam2(client, frame);
    struct FigFrameList list = {0};
    int32_t rc = fig_client_request_and_recv(
        Client_val(client),
        (const uint8_t *)String_val(frame),
        caml_string_length(frame),
        &list);
    if (rc != 0) {
        caml_failwith("fig_client_request_and_recv failed");
    }
    CAMLreturn(bytes_of_frame_list(list));
}

CAMLprim value caml_fig_client_subscribe(value client, value frame) {
    CAMLparam2(client, frame);
    CAMLlocal2(snap, pair);
    struct FigFrameList list = {0};
    struct FigSubHandle *sub = NULL;
    int32_t rc = fig_client_subscribe(
        Client_val(client),
        (const uint8_t *)String_val(frame),
        caml_string_length(frame),
        &list,
        &sub);
    if (rc != 0) {
        caml_failwith("fig_client_subscribe failed");
    }
    snap = bytes_of_frame_list(list);
    pair = caml_alloc_tuple(2);
    Store_field(pair, 0, snap);
    Store_field(pair, 1, alloc_ptr(sub, &fig_sub_ops));
    CAMLreturn(pair);
}

CAMLprim value caml_fig_client_sub_next(value sub, value timeout_ms) {
    CAMLparam2(sub, timeout_ms);
    CAMLlocal1(some);
    struct FigBuffer out = {0};
    int32_t rc = fig_client_sub_next(
        Sub_val(sub), (uint32_t)Int_val(timeout_ms), &out);
    if (rc == 1) {
        caml_failwith("timeout");
    }
    if (rc == 2) {
        CAMLreturn(Val_int(0));
    }
    if (rc != 0) {
        caml_failwith("fig_client_sub_next failed");
    }
    some = caml_alloc(1, 0);
    Store_field(some, 0, bytes_of_fig_buffer(out));
    CAMLreturn(some);
}

CAMLprim value caml_fig_client_sub_close(value sub) {
    CAMLparam1(sub);
    struct FigSubHandle *h = Sub_val(sub);
    if (h != NULL) {
        fig_client_sub_close(h);
        Sub_val(sub) = NULL;
    }
    CAMLreturn(Val_unit);
}
