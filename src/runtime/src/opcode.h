#pragma once
#include <cstdint>

enum class OpCode : uint8_t {
    TextBegin     = 0x02,
    TextEnd       = 0x03,
    CenterAlign   = 0x04,
    RightAlign    = 0x05,
    LeftAlign     = 0x06,
    ColorText     = 0x07,
    SizeText      = 0x08
};