#include "opcode.h"
#include "vm.h"
#include <vector>
#include <string>
#include <sstream>

std::string WikiVM::render(const uint8_t* data, size_t size) {
    std::ostringstream html;
    size_t offset = 0;

    while (offset < size) {
        OpCode op = static_cast<OpCode>(data[offset++]);
        
        switch (op) {
            case OpCode::TextBegin: {
                std::string text = read_string(data, offset);
                expect_end(data, size, offset);
                html << "<p>" << text << "</p>\n";
                break;
            }
            case OpCode::CenterAlign: {
                if (offset < size && static_cast<OpCode>(data[offset]) == OpCode::TextBegin) {
                    offset++;
                }
                std::string text = read_string(data, offset);
                expect_end(data, size, offset);
                html << "<div style=\"text-align: center;\">" << text << "</div>\n";
                break;
            }
            case OpCode::RightAlign: {
                if (offset < size && static_cast<OpCode>(data[offset]) == OpCode::TextBegin) {
                    offset++;
                }
                std::string text = read_string(data, offset);
                expect_end(data, size, offset);
                html << "<div style=\"text-align: right;\">" << text << "</div>\n";
                break;
            }
            case OpCode::LeftAlign: {
                if (offset < size && static_cast<OpCode>(data[offset]) == OpCode::TextBegin) {
                    offset++;
                }
                std::string text = read_string(data, offset);
                expect_end(data, size, offset);
                html << "<div style=\"text-align: left;\">" << text << "</div>\n";
                break;
            }
            default:
                break;
        }
    }

    return html.str();
}

std::string WikiVM::read_string(const uint8_t* data, size_t& offset) {
    uint8_t len = data[offset++];
    std::string str(reinterpret_cast<const char*>(&data[offset]), len);
    offset += len;
    return str;
}

void WikiVM::expect_end(const uint8_t* data, size_t size, size_t& offset) {
    if (offset < size && static_cast<OpCode>(data[offset]) == OpCode::TextEnd) {
        offset++;
    }
}