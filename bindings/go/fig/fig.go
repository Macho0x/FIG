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
	slice := unsafe.Slice(buf.data, buf.len)
	out := make([]byte, len(slice))
	copy(out, slice)
	C.fig_buffer_free(buf)
	return Buffer{data: out}, nil
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

// ChannelStreamID maps a channel id to QUIC stream id (client/server).
func ChannelStreamID(channelID uint16, isServer bool) uint64 {
	var flag C.uint8_t
	if isServer {
		flag = 1
	}
	return uint64(C.fig_client_channel_stream_id(C.uint16_t(channelID), flag))
}
