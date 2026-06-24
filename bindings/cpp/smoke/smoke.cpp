#include "fig/fig.hpp"

int main() {
    auto buf = fig::sbe_encode_new_order_single("CONF-001", "AAPL", true, 100.0, 50.25);
    return buf.bytes().empty() ? 1 : 0;
}
