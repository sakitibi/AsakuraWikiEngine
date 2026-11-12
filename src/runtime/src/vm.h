#pragma once

#include <string>
#include <cstdint>

class WikiVM {
    public:
        static std::string render(const std::string design_color, const uint8_t* data, size_t size);
    private:
        inline static const std::string open_icon = "<path fill=\"currentColor\" d=\"M384 32H64C28.7 32 0 60.7 0 96v320c0 35.3 28.7 64 64 64h320c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM320 272H128c-13.3 0-24-10.7-24-24s10.7-24 24-24h192c13.3 0 24 10.7 24 24s-10.7 24-24 24z\"></path>";
        inline static const std::string close_icon = "<path fill=\"currentColor\" d=\"M64 32C28.7 32 0 60.7 0 96L0 416c0 35.3 28.7 64 64 64l320 0c35.3 0 64-28.7 64-64l0-320c0-35.3-28.7-64-64-64L64 32zM200 344l0-64-64 0c-13.3 0-24-10.7-24-24s10.7-24 24-24l64 0 0-64c0-13.3 10.7-24 24-24s24 10.7 24 24l0 64 64 0c13.3 0 24 10.7 24 24s-10.7 24-24 24l-64 0 0 64c0 13.3-10.7c24-24 c24s-24-10.7-24-24z\"></path>";

        static std::string render_node(const std::string design_color, const uint8_t* data, size_t size, size_t& offset);
        static std::string read_string(const uint8_t* data, size_t& offset);
        static void expect_end(const uint8_t* data, size_t size, size_t& offset);
};