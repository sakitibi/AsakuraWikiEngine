#include "opcode.hpp"
#include <vector>
#include <string>
#include <sstream>

class WikiVM {
public:
    static std::string render(const uint8_t* data, size_t size) {
        std::ostringstream html;
        size_t offset = 0;

        while (offset < size) {
            OpCode op = static_cast<OpCode>(data[offset++]);
            
            switch (op) {
                default:
                    // 未知のオペコードの場合はスキップまたはエラー処理
                    break;
            }
        }

        return html.str();
    }

private:
    static std::string read_string(const uint8_t* data, size_t& offset) {
        uint8_t len = data[offset++];
        std::string str(reinterpret_cast<const char*>(&data[offset]), len);
        offset += len;
        return str;
    }
};