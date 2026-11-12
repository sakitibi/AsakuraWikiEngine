#include "opcode.h"
#include "vm.h"
#include <vector>
#include <string>
#include <sstream>

std::string WikiVM::render(const std::string design_color, const uint8_t* data, size_t size) {
    std::ostringstream html;
    size_t offset = 0;

    while (offset < size) {
        html << render_node(design_color, data, size, offset);
    }

    return html.str();
}

std::string WikiVM::render_node(const std::string design_color, const uint8_t* data, size_t size, size_t& offset) {
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
            std::string innerHtml = render_node(design_color, data, size, offset);
            ss << "<div style=\"text-align: center;\">" << innerHtml << "</div>\n";
            return ss.str();
        }
        case OpCode::RightAlign: {
            std::string innerHtml = render_node(design_color, data, size, offset);
            ss << "<div style=\"text-align: right;\">" << innerHtml << "</div>\n";
            return ss.str();
        }
        case OpCode::LeftAlign: {
            std::string innerHtml = render_node(design_color, data, size, offset);
            ss << "<div style=\"text-align: left;\">" << innerHtml << "</div>\n";
            return ss.str();
        }
        case OpCode::ColorText: {
            std::string color = read_string(data, offset);
            std::string bgColor = read_string(data, offset);
            
            std::string innerHtml = render_node(design_color, data, size, offset);

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
            
            std::string innerHtml = render_node(design_color, data, size, offset);

            ss << "<span style=\"font-size: " << sizeVal << "px;\">" << innerHtml << "</span>";
            return ss.str();
        }
        case OpCode::Heading1: {
            std::string anchor = read_string(data, offset);
            std::string innerHtml = render_node(design_color, data, size, offset);
            if (!anchor.empty()) {
                ss << "<a href=\"#" << anchor << "\"><h2 class=\"header_" << design_color << "\">" << innerHtml << "</h2></a>\n";
            } else {
                ss << "<h2 class=\"header_" << design_color << "\">" << innerHtml << "</h2>\n";
            }
            return ss.str();
        }
        case OpCode::Heading2: {
            std::string anchor = read_string(data, offset);
            std::string innerHtml = render_node(design_color, data, size, offset);
            if (!anchor.empty()) {
                ss << "<a href=\"#" << anchor << "\"><h3 class=\"header_" << design_color << "\">" << innerHtml << "</h3></a>\n";
            } else {
                ss << "<h3 class=\"header_" << design_color << "\">" << innerHtml << "</h3>\n";
            }
            return ss.str();
        }
        case OpCode::Accordion: {
            uint8_t headingLevel = data[offset++];
            bool isOpen = (data[offset++] != 0);
            
            std::string titleHtml = render_node(design_color, data, size, offset);

            uint8_t count = data[offset++];
            std::ostringstream innerHtml;
            for (uint8_t i = 0; i < count; ++i) {
                innerHtml << render_node(design_color, data, size, offset);
            }

            int hNum = 2;
            if (headingLevel >= 1 && headingLevel <= 3) {
                hNum = headingLevel + 1;
            }

            std::string stateClass = isOpen ? "accordion-open" : "accordion-close";

            ss << "<div class=\"accordion-container\">\n"
               << "  <h" << hNum << " class=\"accordion-header header_" << design_color << "\">\n"
               << "    <svg class=\"accordion-icon icon-open\" aria-hidden=\"true\" focusable=\"false\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 448 512\" style=\"width: 1em; height: 1em;\">" << open_icon << "</svg>\n"
               << "    <svg class=\"accordion-icon icon-close\" aria-hidden=\"true\" focusable=\"false\" role=\"img\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 448 512\" style=\"width: 1em; height: 1em;\">" << close_icon << "</svg>\n"
               << "    " << titleHtml << "\n"
               << "  </h" << hNum << ">\n"
               << "  <div class=\"accordion-content" << stateClass << "\">\n"
               << "    <div class=\"accordion-content-inner\">\n"
               << innerHtml.str()
               << "    </div>\n"
               << "  </div>\n"
               << "</div>\n";
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