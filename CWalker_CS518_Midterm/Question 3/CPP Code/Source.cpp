#include <cctype>
#include <fstream>
#include <iostream>
#include <optional>
#include <stdexcept>
#include <string>
#include <string_view>
#include <filesystem>

std::string make_output_name(const std::string& input_name) {
    std::filesystem::path p(input_name);
    return (p.parent_path() / (p.stem().string() + "_resulting_output.txt")).string();
}

std::optional<std::string> clean_line(std::string_view raw) {
    std::size_t start = 0;
    while (start < raw.size() &&
        std::isspace(static_cast<unsigned char>(raw[start]))) {
        ++start;
    }

    std::size_t end = raw.size();
    while (end > start &&
        std::isspace(static_cast<unsigned char>(raw[end - 1]))) {
        --end;
    }

    if (start == end) {
        return std::nullopt;
    }

    return std::string(raw.substr(start, end - start));
}

class LineFormatter {
public:
    virtual ~LineFormatter() = default;
    virtual std::string format(std::size_t line_number,
        const std::string& line) const = 0;
};

class NumberedLineFormatter final : public LineFormatter {
public:
    std::string format(std::size_t line_number,
        const std::string& line) const override {
        return std::to_string(line_number) + ": " + line;
    }
};

void process_stream(std::istream& in,
    std::ostream& out,
    const LineFormatter& formatter) {
    std::string raw_line;
    std::size_t line_number = 1;

    while (std::getline(in, raw_line)) {
        if (auto cleaned = clean_line(raw_line)) {
            out << formatter.format(line_number, *cleaned) << '\n';
            if (!out) {
                throw std::runtime_error("Write failed.");
            }
            ++line_number;
        }
    }

    if (!in.eof()) {
        throw std::runtime_error("Read failed.");
    }
}

void rewrite_file(const std::string& input_name,
    const LineFormatter& formatter) {
    std::ifstream input(input_name);
    if (!input) {
        throw std::runtime_error("Could not open input file: " + input_name);
    }

    const std::string output_name = make_output_name(input_name);
    std::ofstream output(output_name);
    if (!output) {
        throw std::runtime_error("Could not open output file: " + output_name);
    }

    process_stream(input, output, formatter);
}

int main() {
    NumberedLineFormatter formatter;

    try {
        rewrite_file("input.txt", formatter);
        std::cout << "Done.\n";
    }
    catch (const std::exception& e) {
        std::cerr << "Error: " << e.what() << '\n';
        return 1;
    }
}