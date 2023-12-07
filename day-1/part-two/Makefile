TARGET = a.out

SRC = main.cpp

OBJ = $(SRC:.cpp=.o)

CXX = g++

CXXFLAGS = -Wall -Wextra -Werror

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CXX) $(CXXFLAGS) $(OBJ) -o $@

.cpp.o:
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)

re: clean all

.PHONY: all clean re
