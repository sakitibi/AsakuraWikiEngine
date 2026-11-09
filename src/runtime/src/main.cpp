#include <emscripten/emscripten.h>
#include <string>
#include "vm.cpp"

extern "C" {
    // JavaScriptから呼び出されるメイン関数
    EMSCRIPTEN_KEEPALIVE
    const char* render_wiki(const uint8_t* bytecode, size_t size) {
        // 静的変数として保持することで、ポインタを安全にJavaScript側に返す
        static std::string output_html;
        
        output_html = WikiVM::render(bytecode, size);
        return output_html.c_str();
    }
}