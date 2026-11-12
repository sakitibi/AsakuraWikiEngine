#include <emscripten/emscripten.h>
#include <string>
#include "vm.cpp"

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    const char* render_wiki(char* design_color_ptr, const uint8_t* bytecode, size_t size) {
        std::string design_color = design_color_ptr ? design_color_ptr : "default";
        static std::string output_html;
        output_html = WikiVM::render(design_color, bytecode, size);
        return output_html.c_str();
    }
}