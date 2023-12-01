
#include <cctype>
#include <cstdio>
#include <fstream>
#include <iostream>
#include <string>
#include <string_view>
#include <utility>
const std::pair<std::string, int> numbers[9] = {
    {"one", 1}, {"two", 2},   {"three", 3}, {"four", 4}, {"five", 5},
    {"six", 6}, {"seven", 7}, {"eight", 8}, {"nine", 9}};

int match_number(std::string_view view) {
  for (const auto &[word, number] : numbers) {
    if (view.rfind(word, 0) == 0) {
      return number;
    }
  }
  return -1;
}

int main() {
  const auto file_name = "phase1.txt";
  std::ifstream file(file_name);
  std::string line;

  int sum = 0;

  if (file.is_open()) {
    while (std::getline(file, line)) {
      int first_num = -1;
      int last_num = 0;

      for (int i = 0; i < line.size(); ++i) {
        char c = line[i];
        if (std::isdigit(c)) {
          if (first_num == -1) {
            first_num = c - '0';
          }
          last_num = c - '0';
        } else {
          int number = match_number(line.substr(i, line.size() - 1));
          if (number != -1) {
            if (first_num == -1) {
              first_num = number;
            }
            last_num = number;
          }
        }
      }

      sum += first_num * 10 + last_num;
    }
  }

  std::cout << sum << std::endl;

  return 0;
}