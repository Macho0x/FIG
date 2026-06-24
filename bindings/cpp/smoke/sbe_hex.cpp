#include "fig/fig.hpp"

#include <cstdlib>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>

static std::string hexEncode(const std::vector<uint8_t>& bytes) {
    static const char* digits = "0123456789abcdef";
    std::string out;
    out.reserve(bytes.size() * 2);
    for (uint8_t b : bytes) {
        out.push_back(digits[b >> 4]);
        out.push_back(digits[b & 0x0f]);
    }
    return out;
}

static std::string extractExpectedHex(const std::string& json) {
    const std::string id = "\"id\": \"sbe.new_order_single.limit_buy\"";
    const auto idPos = json.find(id);
    if (idPos == std::string::npos) {
        return {};
    }
    const auto hexKey = json.find("\"expected_hex\"", idPos);
    if (hexKey == std::string::npos) {
        return {};
    }
    const auto quoteStart = json.find('"', hexKey + 14);
    if (quoteStart == std::string::npos) {
        return {};
    }
    const auto quoteEnd = json.find('"', quoteStart + 1);
    if (quoteEnd == std::string::npos) {
        return {};
    }
    return json.substr(quoteStart + 1, quoteEnd - quoteStart - 1);
}

int main() {
    auto buf = fig::sbe_encode_new_order_single("CONF-001", "AAPL", true, 100, 50.25);
    const auto& bytes = buf.bytes();
    if (bytes.empty()) {
        std::cerr << "empty SBE payload\n";
        return 1;
    }

    const char* root = std::getenv("FIG_REPO_ROOT");
    if (!root) {
        std::cerr << "FIG_REPO_ROOT not set\n";
        return 1;
    }
    std::ifstream in(std::string(root) + "/tests/conformance/vectors/v1.json");
    if (!in) {
        std::cerr << "open v1.json failed\n";
        return 1;
    }
    std::stringstream ss;
    ss << in.rdbuf();
    const std::string expected = extractExpectedHex(ss.str());
    const std::string got = hexEncode(bytes);
    if (expected.empty() || got != expected) {
        std::cerr << "SBE hex mismatch\nexpected: " << expected << "\ngot:      " << got << "\n";
        return 1;
    }
    std::cout << "fig-cpp sbe hex OK\n";
    return 0;
}
