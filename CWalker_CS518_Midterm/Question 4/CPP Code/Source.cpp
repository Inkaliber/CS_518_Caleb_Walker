#include <iostream>
#include <string>
#include <vector>
#include "Dedup.h"

int main() {
    std::vector<int> a{ 3, 1, 3, 2, 1, 4, 2 };
    auto deduped_a = stable_dedup(a);

    std::cout << "Duped ints: ";
    for (const auto& x : a) {
        std::cout << x << ' ';
    }
    std::cout << '\n';

    std::cout << "Deduped ints: ";
    for (const auto& x : deduped_a) {
        std::cout << x << ' ';
    }
    std::cout << '\n';
    std::cout << '\n';

    std::vector<std::string> b{ "cat", "dog", "cat", "bird", "dog" };
    auto deduped_b = stable_dedup(b);

    std::cout << "Duped strings: ";
    for (const auto& x : b) {
        std::cout << x << ' ';
    }
    std::cout << '\n';

    std::cout << "Deduped strings: ";
    for (const auto& x : deduped_b) {
        std::cout << x << ' ';
    }
    std::cout << '\n';
    std::cout << '\n';

    return 0;
}