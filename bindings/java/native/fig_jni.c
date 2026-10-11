#include <jni.h>
#include <stdint.h>
#include <string.h>

#include "../../../crates/fig-ffi/include/fig.h"

static char *copy_jstring(JNIEnv *env, jstring value) {
    if (value == NULL) {
        return NULL;
    }
    const char *utf = (*env)->GetStringUTFChars(env, value, NULL);
    if (utf == NULL) {
        return NULL;
    }
    char *out = strdup(utf);
    (*env)->ReleaseStringUTFChars(env, value, utf);
    return out;
}

static jint set_out_buffer(JNIEnv *env, struct FigBuffer out, jobjectArray out_arr) {
    jbyteArray arr = (*env)->NewByteArray(env, (jsize)out.len);
    if (arr == NULL) {
        fig_buffer_free(out);
        return -2;
    }
    (*env)->SetByteArrayRegion(env, arr, 0, (jsize)out.len, (const jbyte *)out.data);
    fig_buffer_free(out);
    (*env)->SetObjectArrayElement(env, out_arr, 0, arr);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientConnect(
    JNIEnv *env,
    jclass cls,
    jstring addr,
    jstring server_name,
    jlongArray out_handle) {
    (void)cls;
    char *addr_c = copy_jstring(env, addr);
    char *server_c = copy_jstring(env, server_name);
    if (addr_c == NULL) {
        free(server_c);
        return -1;
    }
    struct FigClientHandle *handle = NULL;
    int32_t rc = fig_client_connect(addr_c, server_c, &handle);
    free(addr_c);
    free(server_c);
    if (rc != 0) {
        return rc;
    }
    jlong ptr = (jlong)(intptr_t)handle;
    (*env)->SetLongArrayRegion(env, out_handle, 0, 1, &ptr);
    return 0;
}

JNIEXPORT void JNICALL Java_fig_FigNative_figClientClose(JNIEnv *env, jclass cls, jlong handle) {
    (void)env;
    (void)cls;
    fig_client_close((struct FigClientHandle *)(intptr_t)handle);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientPing(JNIEnv *env, jclass cls, jlong handle) {
    (void)env;
    (void)cls;
    return fig_client_ping((struct FigClientHandle *)(intptr_t)handle);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientSubscribe(
    JNIEnv *env,
    jclass cls,
    jlong handle,
    jbyteArray frame,
    jobjectArray snapshot_out,
    jlongArray sub_out) {
    (void)cls;
    if (frame == NULL || snapshot_out == NULL || sub_out == NULL) {
        return -1;
    }
    jsize len = (*env)->GetArrayLength(env, frame);
    jbyte *bytes = (*env)->GetByteArrayElements(env, frame, NULL);
    if (bytes == NULL) {
        return -1;
    }
    struct FigFrameList list = {0};
    struct FigSubHandle *sub = NULL;
    int32_t rc = fig_client_subscribe(
        (struct FigClientHandle *)(intptr_t)handle,
        (const uint8_t *)bytes,
        (uintptr_t)len,
        &list,
        &sub);
    (*env)->ReleaseByteArrayElements(env, frame, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    jclass byte_arr_cls = (*env)->FindClass(env, "[B");
    jobjectArray frames = (*env)->NewObjectArray(env, (jsize)list.count, byte_arr_cls, NULL);
    if (frames != NULL && list.frames != NULL) {
        for (uintptr_t i = 0; i < list.count; i++) {
            jbyteArray arr = (*env)->NewByteArray(env, (jsize)list.frames[i].len);
            if (arr != NULL && list.frames[i].data != NULL) {
                (*env)->SetByteArrayRegion(
                    env, arr, 0, (jsize)list.frames[i].len, (const jbyte *)list.frames[i].data);
            }
            (*env)->SetObjectArrayElement(env, frames, (jsize)i, arr);
        }
    }
    (*env)->SetObjectArrayElement(env, snapshot_out, 0, frames);
    jlong ptr = (jlong)(intptr_t)sub;
    (*env)->SetLongArrayRegion(env, sub_out, 0, 1, &ptr);
    fig_frame_list_free(list);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientSubNext(
    JNIEnv *env,
    jclass cls,
    jlong sub,
    jint timeout_ms,
    jobjectArray frame_out) {
    (void)cls;
    struct FigBuffer out = {0};
    int32_t rc = fig_client_sub_next(
        (struct FigSubHandle *)(intptr_t)sub, (uint32_t)timeout_ms, &out);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, frame_out);
}

JNIEXPORT void JNICALL Java_fig_FigNative_figClientSubClose(JNIEnv *env, jclass cls, jlong sub) {
    (void)env;
    (void)cls;
    fig_client_sub_close((struct FigSubHandle *)(intptr_t)sub);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figPayloadCompress(
    JNIEnv *env,
    jclass cls,
    jbyteArray data,
    jobjectArray out_arr) {
    (void)cls;
    jsize len = (*env)->GetArrayLength(env, data);
    jbyte *bytes = (*env)->GetByteArrayElements(env, data, NULL);
    if (bytes == NULL) {
        return -1;
    }
    struct FigBuffer out = {0};
    int32_t rc = fig_payload_compress((const uint8_t *)bytes, (uintptr_t)len, &out);
    (*env)->ReleaseByteArrayElements(env, data, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figFrameEncodeSubscribeAuth(
    JNIEnv *env,
    jclass cls,
    jshort channel_id,
    jint stream_seq,
    jstring routing_key,
    jstring channel_path,
    jstring auth_token,
    jobjectArray out_arr) {
    (void)cls;
    char *rk = copy_jstring(env, routing_key);
    char *cp = copy_jstring(env, channel_path);
    char *auth = copy_jstring(env, auth_token);
    struct FigBuffer out = {0};
    int32_t rc = fig_frame_encode_subscribe_auth(
        (uint16_t)channel_id,
        (uint32_t)stream_seq,
        rk,
        cp,
        auth,
        &out);
    free(rk);
    free(cp);
    free(auth);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figJwtEncode(
    JNIEnv *env,
    jclass cls,
    jstring sub,
    jlong exp,
    jstring secret,
    jobjectArray out_arr) {
    (void)cls;
    char *sub_c = copy_jstring(env, sub);
    char *secret_c = copy_jstring(env, secret);
    if (sub_c == NULL || secret_c == NULL) {
        free(sub_c);
        free(secret_c);
        return -1;
    }
    struct FigBuffer out = {0};
    int32_t rc = fig_jwt_encode(sub_c, (uint64_t)exp, secret_c, &out);
    free(sub_c);
    free(secret_c);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figJwtVerifyBearer(
    JNIEnv *env,
    jclass cls,
    jstring token,
    jstring secret) {
    (void)cls;
    char *token_c = copy_jstring(env, token);
    char *secret_c = copy_jstring(env, secret);
    if (token_c == NULL || secret_c == NULL) {
        free(token_c);
        free(secret_c);
        return -1;
    }
    int32_t rc = fig_jwt_verify_bearer(token_c, secret_c);
    free(token_c);
    free(secret_c);
    return rc;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figSbeEncodeNewOrderSingle(
    JNIEnv *env,
    jclass cls,
    jstring cl_ord_id,
    jstring symbol,
    jbyte side_buy,
    jdouble order_qty,
    jdouble price,
    jobjectArray out_arr) {
    (void)cls;
    char *cl = copy_jstring(env, cl_ord_id);
    char *sym = copy_jstring(env, symbol);
    if (cl == NULL || sym == NULL) {
        free(cl);
        free(sym);
        return -1;
    }
    struct FigBuffer out = {0};
    int32_t rc = fig_sbe_encode_new_order_single(
        cl, sym, (uint8_t)side_buy, order_qty, price, -1, -1, &out);
    free(cl);
    free(sym);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

static void put_string(JNIEnv *env, jobjectArray arr, char *s) {
    if (s == NULL) {
        if (arr != NULL) {
            (*env)->SetObjectArrayElement(env, arr, 0, NULL);
        }
        return;
    }
    jstring js = (*env)->NewStringUTF(env, s);
    fig_string_free(s);
    if (arr != NULL) {
        (*env)->SetObjectArrayElement(env, arr, 0, js);
    }
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCborEncodeMassCancelRequest(
    JNIEnv *env,
    jclass cls,
    jstring cl_ord_id,
    jstring account,
    jstring symbol,
    jbyte side,
    jobjectArray out_arr) {
    (void)cls;
    char *cl = copy_jstring(env, cl_ord_id);
    char *acct = copy_jstring(env, account);
    char *sym = copy_jstring(env, symbol);
    if (cl == NULL || acct == NULL) {
        free(cl);
        free(acct);
        free(sym);
        return -1;
    }
    struct FigBuffer out = {0};
    int32_t rc = fig_cbor_encode_mass_cancel_request(cl, acct, sym, (uint8_t)side, &out);
    free(cl);
    free(acct);
    free(sym);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCborDecodeMassCancelRequest(
    JNIEnv *env,
    jclass cls,
    jbyteArray data,
    jobjectArray out_cl,
    jobjectArray out_account,
    jobjectArray out_symbol,
    jintArray out_side) {
    (void)cls;
    jsize len = (*env)->GetArrayLength(env, data);
    jbyte *bytes = (*env)->GetByteArrayElements(env, data, NULL);
    if (bytes == NULL) {
        return -1;
    }
    char *cl = NULL;
    char *account = NULL;
    char *symbol = NULL;
    uint8_t side = 0;
    int32_t rc = fig_cbor_decode_mass_cancel_request(
        (const uint8_t *)bytes, (uintptr_t)len, &cl, &account, &symbol, &side);
    (*env)->ReleaseByteArrayElements(env, data, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out_cl, cl);
    put_string(env, out_account, account);
    put_string(env, out_symbol, symbol);
    jint side_j = (jint)side;
    (*env)->SetIntArrayRegion(env, out_side, 0, 1, &side_j);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCborDecodeTradingSessionStatus(
    JNIEnv *env,
    jclass cls,
    jbyteArray data,
    jintArray out_status,
    jobjectArray out_session) {
    (void)cls;
    jsize len = (*env)->GetArrayLength(env, data);
    jbyte *bytes = (*env)->GetByteArrayElements(env, data, NULL);
    if (bytes == NULL) {
        return -1;
    }
    uint8_t status = 0;
    char *session = NULL;
    int32_t rc = fig_cbor_decode_trading_session_status(
        (const uint8_t *)bytes, (uintptr_t)len, &status, &session);
    (*env)->ReleaseByteArrayElements(env, data, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    jint status_j = (jint)status;
    (*env)->SetIntArrayRegion(env, out_status, 0, 1, &status_j);
    put_string(env, out_session, session);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCborDecodeSecurityStatus(
    JNIEnv *env,
    jclass cls,
    jbyteArray data,
    jobjectArray out_symbol,
    jintArray out_status) {
    (void)cls;
    jsize len = (*env)->GetArrayLength(env, data);
    jbyte *bytes = (*env)->GetByteArrayElements(env, data, NULL);
    if (bytes == NULL) {
        return -1;
    }
    char *symbol = NULL;
    uint8_t status = 0;
    int32_t rc = fig_cbor_decode_security_status(
        (const uint8_t *)bytes, (uintptr_t)len, &symbol, &status);
    (*env)->ReleaseByteArrayElements(env, data, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out_symbol, symbol);
    jint status_j = (jint)status;
    (*env)->SetIntArrayRegion(env, out_status, 0, 1, &status_j);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCborDecodeMassCancelReport(
    JNIEnv *env,
    jclass cls,
    jbyteArray data,
    jobjectArray out_cl,
    jintArray out_total,
    jintArray out_status) {
    (void)cls;
    jsize len = (*env)->GetArrayLength(env, data);
    jbyte *bytes = (*env)->GetByteArrayElements(env, data, NULL);
    if (bytes == NULL) {
        return -1;
    }
    char *cl = NULL;
    uint32_t total = 0;
    uint8_t status = 0;
    int32_t rc = fig_cbor_decode_mass_cancel_report(
        (const uint8_t *)bytes, (uintptr_t)len, &cl, &total, &status);
    (*env)->ReleaseByteArrayElements(env, data, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out_cl, cl);
    jint total_j = (jint)total;
    jint status_j = (jint)status;
    (*env)->SetIntArrayRegion(env, out_total, 0, 1, &total_j);
    (*env)->SetIntArrayRegion(env, out_status, 0, 1, &status_j);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientRequestSessionStatus(
    JNIEnv *env,
    jclass cls,
    jlong handle,
    jshort channel_id,
    jintArray out_status,
    jobjectArray out_session) {
    (void)cls;
    uint8_t status = 0;
    char *session = NULL;
    int32_t rc = fig_client_request_session_status(
        (struct FigClientHandle *)(intptr_t)handle, (uint16_t)channel_id, &status, &session);
    if (rc != 0) {
        return rc;
    }
    jint status_j = (jint)status;
    (*env)->SetIntArrayRegion(env, out_status, 0, 1, &status_j);
    put_string(env, out_session, session);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientRequestSecurityStatus(
    JNIEnv *env,
    jclass cls,
    jlong handle,
    jstring symbol,
    jshort channel_id,
    jobjectArray out_symbol,
    jintArray out_status) {
    (void)cls;
    char *sym = copy_jstring(env, symbol);
    if (sym == NULL) {
        return -1;
    }
    char *out_sym = NULL;
    uint8_t status = 0;
    int32_t rc = fig_client_request_security_status(
        (struct FigClientHandle *)(intptr_t)handle,
        sym,
        (uint16_t)channel_id,
        &out_sym,
        &status);
    free(sym);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out_symbol, out_sym);
    jint status_j = (jint)status;
    (*env)->SetIntArrayRegion(env, out_status, 0, 1, &status_j);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientMassCancel(
    JNIEnv *env,
    jclass cls,
    jlong handle,
    jstring cl_ord_id,
    jstring account,
    jstring symbol,
    jbyte side,
    jshort channel_id,
    jobjectArray out_cl,
    jintArray out_total,
    jintArray out_status) {
    (void)cls;
    char *cl = copy_jstring(env, cl_ord_id);
    char *acct = copy_jstring(env, account);
    char *sym = copy_jstring(env, symbol);
    if (cl == NULL || acct == NULL) {
        free(cl);
        free(acct);
        free(sym);
        return -1;
    }
    char *out_id = NULL;
    uint32_t total = 0;
    uint8_t status = 0;
    int32_t rc = fig_client_mass_cancel(
        (struct FigClientHandle *)(intptr_t)handle,
        cl,
        acct,
        sym,
        (uint8_t)side,
        (uint16_t)channel_id,
        &out_id,
        &total,
        &status);
    free(cl);
    free(acct);
    free(sym);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out_cl, out_id);
    jint total_j = (jint)total;
    jint status_j = (jint)status;
    (*env)->SetIntArrayRegion(env, out_total, 0, 1, &total_j);
    (*env)->SetIntArrayRegion(env, out_status, 0, 1, &status_j);
    return 0;
}
