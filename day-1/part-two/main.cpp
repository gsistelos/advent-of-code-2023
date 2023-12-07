#include <cstring>
#include <fstream>
#include <iostream>

using namespace std;

#define NPOS 255

string numbers[] = {"one", "two",   "three", "four", "five",
                    "six", "seven", "eight", "nine"};

unsigned char super_isdigit(const char *str) {
  if (isdigit(str[0])) {
    return str[0] - '0';
  }

  for (size_t i = 0; i < 9; i++) {
    if (strncmp(str, numbers[i].c_str(), numbers[i].size()) == 0) {
      return i + 1;
    }
  }
  return NPOS;
}

int main(int argc, char **argv) {
  if (argc != 2) {
    cerr << "Usage: " << argv[0] << " <filename>" << endl;
    return 1;
  }

  ifstream file(argv[1]);
  if (!file.is_open()) {
    cerr << "Error: " << argv[1] << ": " << std::string(strerror(errno))
         << endl;
    return 1;
  }

  unsigned int answer = 0;

  string line;

  while (getline(file, line)) {
    unsigned char digit[2];

    // Iterate from first to last
    for (string::iterator it = line.begin(); it != line.end(); ++it) {
      digit[0] = super_isdigit(&*it);
      if (digit[0] != NPOS) {
        break;
      }
    }

    // Iterate from last to first
    for (string::iterator it = line.end() - 1; it != line.begin() - 1; --it) {
      digit[1] = super_isdigit(&*it);
      if (digit[1] != NPOS) {
        break;
      }
    }

    /* cout << (int)digit[0] << (int)digit[1] << endl; */

    answer += digit[0] * 10 + digit[1];
  }

  cout << "Answer: " << answer << endl;

  return 0;
}
