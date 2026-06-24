// Package fig wraps the FIG stable C ABI (fig.h) for Go clients.
package fig

/*
#cgo LDFLAGS: -L${SRCDIR}/../../../target/debug -lfig_ffi
#include "../../../crates/fig-ffi/include/fig.h"
#include <stdlib.h>
*/
import "C"
import (
	"errors"
	"unsafe"
)

// Version returns the linked FIG FFI version string.
func Version() string {
	return C.GoString(C.fig_version())
}

// Buffer holds bytes returned by the FFI layer.
type Buffer struct {
	data []byte
}

func (b *Buffer) Bytes() []byte { return b.data }

func intoGoBuffer(buf C.struct_FigBuffer) (Buffer, error) {
	if buf.data == nil || buf.len == 0 {
		C.fig_buffer_free(buf)
		return Buffer{}, errors.New("empty fig buffer")
	}
	out := C.GoBytes(unsafe.Pointer(buf.data), C.int(buf.len))
	C.fig_buffer_free(buf)
	return Buffer{data: out}, nil
}

// Client is a connected FIG client over TREE.
type Client struct {
	handle *C.struct_FigClientHandle
}

// Connect opens a TREE connection to a FIG server (`host:port`).
func Connect(addr, serverName string) (*Client, error) {
	cAddr := C.CString(addr)
	defer C.free(unsafe.Pointer(cAddr))
	var cName *C.char
	if serverName != "" {
		cName = C.CString(serverName)
		defer C.free(unsafe.Pointer(cName))
	}
	var out *C.struct_FigClientHandle
	if rc := C.fig_client_connect(cAddr, cName, &out); rc != 0 {
		return nil, errors.New("fig_client_connect failed")
	}
	return &Client{handle: out}, nil
}

// Close releases the client connection.
func (c *Client) Close() {
	if c == nil || c.handle == nil {
		return
	}
	C.fig_client_close(c.handle)
	c.handle = nil
}

// Ping sends a PING control frame on channel 0.
func (c *Client) Ping() error {
	if c == nil || c.handle == nil {
		return errors.New("client closed")
	}
	if rc := C.fig_client_ping(c.handle); rc != 0 {
		return errors.New("fig_client_ping failed")
	}
	return nil
}

// RequestAndRecv sends encoded frame bytes and returns response frame bytes.
func (c *Client) RequestAndRecv(frame []byte) ([][]byte, error) {
	if c == nil || c.handle == nil {
		return nil, errors.New("client closed")
	}
	if len(frame) == 0 {
		return nil, errors.New("empty frame")
	}
	var list C.struct_FigFrameList
	rc := C.fig_client_request_and_recv(
		c.handle,
		(*C.uint8_t)(unsafe.Pointer(&frame[0])),
		C.uintptr_t(len(frame)),
		&list,
	)
	if rc != 0 {
		return nil, errors.New("fig_client_request_and_recv failed")
	}
	defer C.fig_frame_list_free(list)
	if list.frames == nil || list.count == 0 {
		return nil, nil
	}
	slice := unsafe.Slice(list.frames, list.count)
	out := make([][]byte, len(slice))
	for i, buf := range slice {
		if buf.data == nil || buf.len == 0 {
			continue
		}
		out[i] = C.GoBytes(unsafe.Pointer(buf.data), C.int(buf.len))
	}
	return out, nil
}

// EncodeRequestFrame builds a native FIG REQUEST frame.
func EncodeRequestFrame(channelID uint16, streamSeq uint32, schemaID uint8, channelPath, method string, payload []byte) (Buffer, error) {
	cPath := C.CString(channelPath)
	cMethod := C.CString(method)
	defer C.free(unsafe.Pointer(cPath))
	defer C.free(unsafe.Pointer(cMethod))
	var p *C.uint8_t
	var plen C.uintptr_t
	if len(payload) > 0 {
		p = (*C.uint8_t)(unsafe.Pointer(&payload[0]))
		plen = C.uintptr_t(len(payload))
	}
	var out C.struct_FigBuffer
	rc := C.fig_frame_encode_request_ex(
		C.uint16_t(channelID),
		C.uint32_t(streamSeq),
		C.uint8_t(schemaID),
		cPath,
		cMethod,
		nil,
		p,
		plen,
		&out,
	)
	if rc != 0 {
		return Buffer{}, errors.New("fig_frame_encode_request_ex failed")
	}
	return intoGoBuffer(out)
}

// EncodeRequestFrameAuth builds REQUEST with optional auth token.
func EncodeRequestFrameAuth(channelID uint16, streamSeq uint32, schemaID uint8, channelPath, method, contentType, authToken string, payload []byte) (Buffer, error) {
	cPath := cStringOrNil(channelPath)
	cMethod := cStringOrNil(method)
	cCT := cStringOrNil(contentType)
	cAuth := cStringOrNil(authToken)
	defer freeCString(cPath)
	defer freeCString(cMethod)
	defer freeCString(cCT)
	defer freeCString(cAuth)
	var p *C.uint8_t
	var plen C.uintptr_t
	if len(payload) > 0 {
		p = (*C.uint8_t)(unsafe.Pointer(&payload[0]))
		plen = C.uintptr_t(len(payload))
	}
	var out C.struct_FigBuffer
	rc := C.fig_frame_encode_request_auth(
		C.uint16_t(channelID),
		C.uint32_t(streamSeq),
		C.uint8_t(schemaID),
		cPath,
		cMethod,
		cCT,
		cAuth,
		p,
		plen,
		&out,
	)
	if rc != 0 {
		return Buffer{}, errors.New("fig_frame_encode_request_auth failed")
	}
	return intoGoBuffer(out)
}

// EncodeSubscribeFrame builds a SUBSCRIBE frame.
func EncodeSubscribeFrame(channelID uint16, streamSeq uint32, routingKey, channelPath string) (Buffer, error) {
	cRK := C.CString(routingKey)
	cPath := C.CString(channelPath)
	defer C.free(unsafe.Pointer(cRK))
	defer C.free(unsafe.Pointer(cPath))
	var out C.struct_FigBuffer
	rc := C.fig_frame_encode_subscribe(
		C.uint16_t(channelID),
		C.uint32_t(streamSeq),
		cRK,
		cPath,
		&out,
	)
	if rc != 0 {
		return Buffer{}, errors.New("fig_frame_encode_subscribe failed")
	}
	return intoGoBuffer(out)
}

// EncodeSubscribeFrameAuth builds SUBSCRIBE with optional auth token.
func EncodeSubscribeFrameAuth(channelID uint16, streamSeq uint32, routingKey, channelPath, authToken string) (Buffer, error) {
	cRK := C.CString(routingKey)
	cPath := C.CString(channelPath)
	cAuth := cStringOrNil(authToken)
	defer C.free(unsafe.Pointer(cRK))
	defer C.free(unsafe.Pointer(cPath))
	defer freeCString(cAuth)
	var out C.struct_FigBuffer
	rc := C.fig_frame_encode_subscribe_auth(
		C.uint16_t(channelID),
		C.uint32_t(streamSeq),
		cRK,
		cPath,
		cAuth,
		&out,
	)
	if rc != 0 {
		return Buffer{}, errors.New("fig_frame_encode_subscribe_auth failed")
	}
	return intoGoBuffer(out)
}

// ChannelStreamID maps a channel id to QUIC stream id (client/server).
func ChannelStreamID(channelID uint16, isServer bool) uint64 {
	var flag C.uint8_t
	if isServer {
		flag = 1
	}
	return uint64(C.fig_client_channel_stream_id(C.uint16_t(channelID), flag))
}

func cStringOrNil(s string) *C.char {
	if s == "" {
		return nil
	}
	return C.CString(s)
}

func freeCString(s *C.char) {
	if s != nil {
		C.free(unsafe.Pointer(s))
	}
}

// JwtEncode builds an HS256 dev JWT for wire AuthToken extensions.
func JwtEncode(sub string, exp uint64, secret string) (string, error) {
	cSub := C.CString(sub)
	cSecret := C.CString(secret)
	defer C.free(unsafe.Pointer(cSub))
	defer C.free(unsafe.Pointer(cSecret))
	var out C.struct_FigBuffer
	if rc := C.fig_jwt_encode(cSub, C.uint64_t(exp), cSecret, &out); rc != 0 {
		return "", errors.New("fig_jwt_encode failed")
	}
	buf, err := intoGoBuffer(out)
	if err != nil {
		return "", err
	}
	return string(buf.Bytes()), nil
}

// JwtDecodeSub verifies a JWT and returns the subject claim.
func JwtDecodeSub(token, secret string) (string, error) {
	cToken := C.CString(token)
	cSecret := C.CString(secret)
	defer C.free(unsafe.Pointer(cToken))
	defer C.free(unsafe.Pointer(cSecret))
	var outSub *C.char
	if rc := C.fig_jwt_decode_sub(cToken, cSecret, &outSub); rc != 0 {
		return "", errors.New("fig_jwt_decode_sub failed")
	}
	defer C.fig_string_free(outSub)
	return C.GoString(outSub), nil
}

// JwtVerifyBearer checks JWT signature and expiry.
func JwtVerifyBearer(token, secret string) error {
	cToken := C.CString(token)
	cSecret := C.CString(secret)
	defer C.free(unsafe.Pointer(cToken))
	defer C.free(unsafe.Pointer(cSecret))
	if rc := C.fig_jwt_verify_bearer(cToken, cSecret); rc != 0 {
		return errors.New("fig_jwt_verify_bearer failed")
	}
	return nil
}

// SbeEncodeNewOrderSingle encodes a limit order as FIG SBE payload bytes.
func SbeEncodeNewOrderSingle(clOrdID, symbol string, sideBuy bool, qty, price float64) (Buffer, error) {
	cCl := C.CString(clOrdID)
	cSym := C.CString(symbol)
	defer C.free(unsafe.Pointer(cCl))
	defer C.free(unsafe.Pointer(cSym))
	var side C.uint8_t
	if sideBuy {
		side = 1
	}
	var out C.struct_FigBuffer
	rc := C.fig_sbe_encode_new_order_single(
		cCl, cSym, side, C.double(qty), C.double(price), -1, -1, &out,
	)
	if rc != 0 {
		return Buffer{}, errors.New("fig_sbe_encode_new_order_single failed")
	}
	return intoGoBuffer(out)
}
