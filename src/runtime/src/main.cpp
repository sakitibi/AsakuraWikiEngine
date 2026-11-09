#include <emscripten/emscripten.h>
#include <string>
#include "vm.cpp"

extern "C" {
    EMSCRIPTEN_KEEPALIVE
    const char* render_wiki(const uint8_t* bytecode, size_t size) {
        static std::string output_html;
        
        output_html = WikiVM::render(bytecode, size);
        return output_html.c_str();
    }
}