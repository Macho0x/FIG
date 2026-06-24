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
