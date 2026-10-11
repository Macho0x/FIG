#include "fig/fig.hpp"

int main() {
    auto buf = fig::sbe_encode_new_order_single("CONF-001", "AAPL", true, 100.0, 50.25);
    if (buf.bytes().empty()) {
        return 1;
    }
    auto body = fig::cbor_encode_mass_cancel_request("MC-001", "DEMO", "AAPL", 1);
    auto decoded = fig::cbor_decode_mass_cancel_request(body.bytes().data(), body.bytes().size());
    if (decoded.cl_ord_id != "MC-001" || decoded.account != "DEMO" || decoded.side != 1) {
        return 1;
    }
    if (!decoded.symbol || *decoded.symbol != "AAPL") {
        return 1;
    }
    return 0;
}
