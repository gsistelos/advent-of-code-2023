#include <cstring>
#include <fstream>
#include <iostream>

using namespace std;

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
      if (isdigit(*it)) {
        digit[0] = *it - '0';
        break;
      }
    }

    // Iterate from last to first
    for (string::iterator it = line.end() - 1; it != line.begin() - 1; --it) {
      if (isdigit(*it)) {
        digit[1] = *it - '0';
        break;
      }
    }

    /* cout << (int)digit[0] << (int)digit[1] << endl; */

    answer += digit[0] * 10 + digit[1];
  }

  cout << "Answer: " << answer << endl;

  return 0;
}
