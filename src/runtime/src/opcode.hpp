#pragma once
#include <cstdint>

enum class OpCode : uint8_t {
    Text        = 0x02,
    CenterAlign = 0x04,
    RightAlign  = 0x05,
    LeftAlign   = 0x06
};