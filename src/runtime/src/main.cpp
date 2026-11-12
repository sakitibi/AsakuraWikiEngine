#include <emscripten/emscripten.h>
#include <string>
#include "vm.cpp"

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    const char* render_wiki(std::string design_color, const uint8_t* bytecode, size_t size) {
        if (design_color == "") {
            design_color = "emerald";
        }
        static std::string output_html;
        
        output_html = WikiVM::render(design_color, bytecode, size);
        return output_html.c_str();
    }
}