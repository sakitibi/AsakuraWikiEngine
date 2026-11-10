#pragma once

#include <string>
#include <cstdint>

class WikiVM {
    public:
        static std::string render(const uint8_t* data, size_t size);
    private:
        static std::string render_node(const uint8_t* data, size_t size, size_t& offset);
        static std::string read_string(const uint8_t* data, size_t& offset);
        static void expect_end(const uint8_t* data, size_t size, size_t& offset);
};