#include <stdint.h>
#include <stdlib.h>

#define JAVA_NEW0(jname, cname)                                                          \
    JNIEXPORT jlong JNICALL Java_fig_FigNative_##jname(JNIEnv *env, jclass cls) {        \
        (void)env;                                                                       \
        (void)cls;                                                                       \
        return (jlong)(intptr_t)cname();                                                 \
    }

#define JAVA_NEW1(jname, cname)                                                                  \
    JNIEXPORT jlong JNICALL Java_fig_FigNative_##jname(JNIEnv *env, jclass cls, jlong n) {       \
        (void)env;                                                                               \
        (void)cls;                                                                               \
        return (jlong)(intptr_t)cname((uintptr_t)n);                                             \
    }

#define JAVA_FREE(jname, cname, ctype)                                                           \
    JNIEXPORT void JNICALL Java_fig_FigNative_##jname(JNIEnv *env, jclass cls, jlong handle) {   \
        (void)env;                                                                               \
        (void)cls;                                                                               \
        if (handle != 0) {                                                                       \
            cname((ctype *)(intptr_t)handle);                                                    \
        }                                                                                        \
    }

#define JAVA_APPLY(jname, cname, ctype)                                                          \
    JNIEXPORT jint JNICALL Java_fig_FigNative_##jname(                                           \
        JNIEnv *env, jclass cls, jlong handle, jbyteArray payload) {                            \
        (void)cls;                                                                               \
        if (handle == 0 || payload == NULL) {                                                   \
            return -1;                                                                           \
        }                                                                                        \
        jsize len = (*env)->GetArrayLength(env, payload);                                       \
        jbyte *bytes = (*env)->GetByteArrayElements(env, payload, NULL);                        \
        if (bytes == NULL) {                                                                     \
            return -1;                                                                           \
        }                                                                                        \
        int32_t rc = cname((ctype *)(intptr_t)handle, (const uint8_t *)bytes, (uintptr_t)len);   \
        (*env)->ReleaseByteArrayElements(env, payload, bytes, JNI_ABORT);                       \
        return rc;                                                                               \
    }

#define JAVA_F64(jname, cname, ctype)                                                            \
    JNIEXPORT jdouble JNICALL Java_fig_FigNative_##jname(JNIEnv *env, jclass cls, jlong handle) {\
        (void)env;                                                                               \
        (void)cls;                                                                               \
        if (handle == 0) {                                                                       \
            return 0;                                                                            \
        }                                                                                        \
        return cname((const ctype *)(intptr_t)handle);                                           \
    }

#define JAVA_U64(jname, cname, ctype)                                                            \
    JNIEXPORT jlong JNICALL Java_fig_FigNative_##jname(JNIEnv *env, jclass cls, jlong handle) {  \
        (void)env;                                                                               \
        (void)cls;                                                                               \
        if (handle == 0) {                                                                       \
            return 0;                                                                            \
        }                                                                                        \
        return (jlong)cname((const ctype *)(intptr_t)handle);                                    \
    }

static jint store_frame_list(JNIEnv *env, struct FigFrameList list, jobjectArray out_holder) {
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
    (*env)->SetObjectArrayElement(env, out_holder, 0, frames);
    fig_frame_list_free(list);
    return 0;
}

JNIEXPORT jstring JNICALL Java_fig_FigNative_figVersion(JNIEnv *env, jclass cls) {
    (void)cls;
    return (*env)->NewStringUTF(env, fig_version());
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figJwtDecodeSub(
    JNIEnv *env, jclass cls, jstring token, jstring secret, jobjectArray out_sub) {
    (void)cls;
    char *token_c = copy_jstring(env, token);
    char *secret_c = copy_jstring(env, secret);
    if (token_c == NULL || secret_c == NULL) {
        free(token_c);
        free(secret_c);
        return -1;
    }
    char *sub = NULL;
    int32_t rc = fig_jwt_decode_sub(token_c, secret_c, &sub);
    free(token_c);
    free(secret_c);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out_sub, sub);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientRequestAndRecv(
    JNIEnv *env, jclass cls, jlong handle, jbyteArray frame, jobjectArray out_frames) {
    (void)cls;
    if (frame == NULL || out_frames == NULL) {
        return -1;
    }
    jsize len = (*env)->GetArrayLength(env, frame);
    jbyte *bytes = (*env)->GetByteArrayElements(env, frame, NULL);
    if (bytes == NULL) {
        return -1;
    }
    struct FigFrameList list = {0};
    int32_t rc = fig_client_request_and_recv(
        (struct FigClientHandle *)(intptr_t)handle, (const uint8_t *)bytes, (uintptr_t)len, &list);
    (*env)->ReleaseByteArrayElements(env, frame, bytes, JNI_ABORT);
    if (rc != 0) {
        return rc;
    }
    return store_frame_list(env, list, out_frames);
}

static jint encode_request(
    JNIEnv *env,
    jshort channel_id,
    jint stream_seq,
    jbyte schema_id,
    jstring channel_path,
    jstring method,
    jstring auth_token,
    jbyteArray payload,
    jobjectArray out_arr,
    int with_auth) {
    char *path = copy_jstring(env, channel_path);
    char *method_c = copy_jstring(env, method);
    char *auth = with_auth ? copy_jstring(env, auth_token) : NULL;
    jbyte *bytes = NULL;
    jsize len = 0;
    if (payload != NULL) {
        len = (*env)->GetArrayLength(env, payload);
        bytes = (*env)->GetByteArrayElements(env, payload, NULL);
        if (bytes == NULL) {
            free(path);
            free(method_c);
            free(auth);
            return -1;
        }
    }
    struct FigBuffer out = {0};
    int32_t rc;
    if (with_auth) {
        rc = fig_frame_encode_request_auth(
            (uint16_t)channel_id,
            (uint32_t)stream_seq,
            (uint8_t)schema_id,
            path,
            method_c,
            NULL,
            auth,
            (const uint8_t *)bytes,
            (uintptr_t)len,
            &out);
    } else {
        rc = fig_frame_encode_request_ex(
            (uint16_t)channel_id,
            (uint32_t)stream_seq,
            (uint8_t)schema_id,
            path,
            method_c,
            NULL,
            (const uint8_t *)bytes,
            (uintptr_t)len,
            &out);
    }
    if (bytes != NULL) {
        (*env)->ReleaseByteArrayElements(env, payload, bytes, JNI_ABORT);
    }
    free(path);
    free(method_c);
    free(auth);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figFrameEncodeRequestEx(
    JNIEnv *env,
    jclass cls,
    jshort channel_id,
    jint stream_seq,
    jbyte schema_id,
    jstring channel_path,
    jstring method,
    jbyteArray payload,
    jobjectArray out_arr) {
    (void)cls;
    return encode_request(
        env, channel_id, stream_seq, schema_id, channel_path, method, NULL, payload, out_arr, 0);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figFrameEncodeRequestAuth(
    JNIEnv *env,
    jclass cls,
    jshort channel_id,
    jint stream_seq,
    jbyte schema_id,
    jstring channel_path,
    jstring method,
    jstring auth_token,
    jbyteArray payload,
    jobjectArray out_arr) {
    (void)cls;
    return encode_request(
        env, channel_id, stream_seq, schema_id, channel_path, method, auth_token, payload, out_arr, 1);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figClientPostBatch(
    JNIEnv *env,
    jclass cls,
    jlong handle,
    jstring account,
    jbyteArray payload,
    jshort channel_id,
    jobjectArray out_arr) {
    (void)cls;
    char *account_c = copy_jstring(env, account);
    if (account_c == NULL || payload == NULL) {
        free(account_c);
        return -1;
    }
    jsize len = (*env)->GetArrayLength(env, payload);
    jbyte *bytes = (*env)->GetByteArrayElements(env, payload, NULL);
    if (bytes == NULL) {
        free(account_c);
        return -1;
    }
    struct FigBuffer out = {0};
    int32_t rc = fig_client_post_batch(
        (struct FigClientHandle *)(intptr_t)handle,
        account_c,
        (const uint8_t *)bytes,
        (uintptr_t)len,
        (uint16_t)channel_id,
        &out);
    (*env)->ReleaseByteArrayElements(env, payload, bytes, JNI_ABORT);
    free(account_c);
    if (rc != 0) {
        return rc;
    }
    return set_out_buffer(env, out, out_arr);
}

JAVA_NEW0(figOrderBookNew, fig_order_book_new)
JAVA_FREE(figOrderBookFree, fig_order_book_free, struct FigOrderBookHandle)
JAVA_APPLY(figOrderBookApplySnapshot, fig_order_book_apply_snapshot, struct FigOrderBookHandle)
JAVA_APPLY(figOrderBookApplyDelta, fig_order_book_apply_delta, struct FigOrderBookHandle)
JAVA_F64(figOrderBookBestBid, fig_order_book_best_bid, struct FigOrderBookHandle)

JAVA_NEW0(figMidsNew, fig_mids_new)
JAVA_FREE(figMidsFree, fig_mids_free, struct FigMidsHandle)
JAVA_APPLY(figMidsApplyTicker, fig_mids_apply_ticker, struct FigMidsHandle)
JAVA_APPLY(figMidsApplyBatch, fig_mids_apply_batch, struct FigMidsHandle)

JNIEXPORT jdouble JNICALL Java_fig_FigNative_figMidsMid(
    JNIEnv *env, jclass cls, jlong handle, jstring symbol) {
    (void)cls;
    if (handle == 0) {
        return 0;
    }
    char *symbol_c = copy_jstring(env, symbol);
    double mid = fig_mids_mid((const struct FigMidsHandle *)(intptr_t)handle, symbol_c);
    free(symbol_c);
    return mid;
}

JAVA_NEW0(figBboNew, fig_bbo_new)
JAVA_FREE(figBboFree, fig_bbo_free, struct FigBboHandle)
JAVA_APPLY(figBboApply, fig_bbo_apply, struct FigBboHandle)
JAVA_F64(figBboImpliedMid, fig_bbo_implied_mid, struct FigBboHandle)
JAVA_F64(figBboBestBid, fig_bbo_best_bid, struct FigBboHandle)
JAVA_F64(figBboBestAsk, fig_bbo_best_ask, struct FigBboHandle)

JAVA_NEW1(figTradeTapeNew, fig_trade_tape_new)
JAVA_FREE(figTradeTapeFree, fig_trade_tape_free, struct FigTradeTapeHandle)
JAVA_APPLY(figTradeTapePush, fig_trade_tape_push, struct FigTradeTapeHandle)
JAVA_U64(figTradeTapeLen, fig_trade_tape_len, struct FigTradeTapeHandle)
JAVA_F64(figTradeTapeLatestPrice, fig_trade_tape_latest_price, struct FigTradeTapeHandle)

JAVA_NEW0(figMarkPriceNew, fig_mark_price_new)
JAVA_FREE(figMarkPriceFree, fig_mark_price_free, struct FigMarkPriceHandle)
JAVA_APPLY(figMarkPriceApply, fig_mark_price_apply, struct FigMarkPriceHandle)

JNIEXPORT jdouble JNICALL Java_fig_FigNative_figMarkPriceGet(
    JNIEnv *env, jclass cls, jlong handle, jstring symbol) {
    (void)cls;
    if (handle == 0) {
        return 0;
    }
    char *symbol_c = copy_jstring(env, symbol);
    double price = fig_mark_price_get((const struct FigMarkPriceHandle *)(intptr_t)handle, symbol_c);
    free(symbol_c);
    return price;
}

JAVA_NEW1(figOrdersNew, fig_orders_new)
JAVA_FREE(figOrdersFree, fig_orders_free, struct FigOrdersHandle)
JAVA_APPLY(figOrdersApplySnapshot, fig_orders_apply_snapshot, struct FigOrdersHandle)
JAVA_APPLY(figOrdersApplyExecution, fig_orders_apply_execution, struct FigOrdersHandle)
JAVA_U64(figOrdersOpenCount, fig_orders_open_count, struct FigOrdersHandle)
JAVA_U64(figOrdersExecutionCount, fig_orders_execution_count, struct FigOrdersHandle)

JAVA_NEW1(figFundingNew, fig_funding_new)
JAVA_FREE(figFundingFree, fig_funding_free, struct FigFundingHandle)
JAVA_APPLY(figFundingApply, fig_funding_apply, struct FigFundingHandle)
JAVA_U64(figFundingLen, fig_funding_len, struct FigFundingHandle)
JAVA_F64(figFundingLatestAmount, fig_funding_latest_amount, struct FigFundingHandle)

JAVA_NEW1(figAggTradesNew, fig_agg_trades_new)
JAVA_FREE(figAggTradesFree, fig_agg_trades_free, struct FigAggTradesHandle)
JAVA_APPLY(figAggTradesApply, fig_agg_trades_apply, struct FigAggTradesHandle)
JAVA_U64(figAggTradesLen, fig_agg_trades_len, struct FigAggTradesHandle)
JAVA_F64(figAggTradesLatestPrice, fig_agg_trades_latest_price, struct FigAggTradesHandle)

JAVA_NEW1(figLedgerNew, fig_ledger_new)
JAVA_FREE(figLedgerFree, fig_ledger_free, struct FigLedgerHandle)
JAVA_APPLY(figLedgerApply, fig_ledger_apply, struct FigLedgerHandle)
JAVA_U64(figLedgerLen, fig_ledger_len, struct FigLedgerHandle)

JAVA_NEW1(figLiquidationNew, fig_liquidation_new)
JAVA_FREE(figLiquidationFree, fig_liquidation_free, struct FigLiquidationHandle)
JAVA_APPLY(figLiquidationApplyUser, fig_liquidation_apply_user, struct FigLiquidationHandle)
JAVA_APPLY(figLiquidationApplyPublic, fig_liquidation_apply_public, struct FigLiquidationHandle)
JAVA_U64(figLiquidationUserCount, fig_liquidation_user_count, struct FigLiquidationHandle)

JAVA_NEW0(figCandleNew, fig_candle_new)
JAVA_FREE(figCandleFree, fig_candle_free, struct FigCandleHandle)
JAVA_APPLY(figCandleApply, fig_candle_apply, struct FigCandleHandle)
JAVA_F64(figCandleClose, fig_candle_close, struct FigCandleHandle)

JNIEXPORT jint JNICALL Java_fig_FigNative_figCandleIsFinal(JNIEnv *env, jclass cls, jlong handle) {
    (void)env;
    (void)cls;
    if (handle == 0) {
        return -1;
    }
    return fig_candle_is_final((const struct FigCandleHandle *)(intptr_t)handle);
}

static jint candle_text(
    JNIEnv *env,
    jlong handle,
    jobjectArray out,
    int32_t (*fn)(const struct FigCandleHandle *, char **)) {
    char *text = NULL;
    int32_t rc = fn((const struct FigCandleHandle *)(intptr_t)handle, &text);
    if (rc != 0) {
        return rc;
    }
    put_string(env, out, text);
    return 0;
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCandleSymbol(
    JNIEnv *env, jclass cls, jlong handle, jobjectArray out) {
    (void)cls;
    return candle_text(env, handle, out, fig_candle_symbol);
}

JNIEXPORT jint JNICALL Java_fig_FigNative_figCandleInterval(
    JNIEnv *env, jclass cls, jlong handle, jobjectArray out) {
    (void)cls;
    return candle_text(env, handle, out, fig_candle_interval);
}
