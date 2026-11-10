#include "opcode.h"
#include "vm.h"
#include <vector>
#include <string>
#include <sstream>

std::string WikiVM::render(const uint8_t* data, size_t size) {
    std::ostringstream html;
    size_t offset = 0;

    while (offset < size) {
        html << render_node(data, size, offset);
    }

    return html.str();
}

std::string WikiVM::render_node(const uint8_t* data, size_t size, size_t& offset) {
    if (offset >= size) return "";

    OpCode op = static_cast<OpCode>(data[offset++]);
    std::ostringstream ss;

    switch (op) {
        case OpCode::TextBegin: {
            std::string text = read_string(data, offset);
            expect_end(data, size, offset);
            return text;
        }
        case OpCode::CenterAlign: {
            std::string innerHtml = render_node(data, size, offset);
            ss << "<div style=\"text-align: center;\">" << innerHtml << "</div>\n";
            return ss.str();
        }
        case OpCode::RightAlign: {
            std::string innerHtml = render_node(data, size, offset);
            ss << "<div style=\"text-align: right;\">" << innerHtml << "</div>\n";
            return ss.str();
        }
        case OpCode::LeftAlign: {
            std::string innerHtml = render_node(data, size, offset);
            ss << "<div style=\"text-align: left;\">" << innerHtml << "</div>\n";
            return ss.str();
        }
        case OpCode::ColorText: {
            std::string color = read_string(data, offset);
            std::string bgColor = read_string(data, offset);
            
            std::string innerHtml = render_node(data, size, offset);

            ss << "<span style=\"";
            if (!color.empty()) {
                ss << "color: " << color << "; ";
            }
            if (!bgColor.empty()) {
                ss << "background-color: " << bgColor << "; ";
            }
            ss << "\">" << innerHtml << "</span>";
            return ss.str();
        }
        case OpCode::SizeText: {
            std::string sizeVal = read_string(data, offset);
            
            std::string innerHtml = render_node(data, size, offset);

            ss << "<span style=\"font-size: " << sizeVal << ";\">" << innerHtml << "</span>";
            return ss.str();
        }
        default:
            return "";
    }
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