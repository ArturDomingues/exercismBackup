#include "raindrops.h"
#include <string>

namespace raindrops {
    std::string convert(int value){
        std::string answer{};
        if (value % 3 == 0) {
            answer += "Pling";
        }
        if (value % 5 == 0) {
            answer += "Plang";
        }
        if (value % 7 == 0) {
            answer += "Plong";
        }
        return answer.empty() ? std::to_string(value) : answer;
    }
}  // namespace raindrops
