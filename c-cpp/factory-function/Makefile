CXX := g++
CXXFLAGS := -std=c++17 -Wall -g

all: point animal

point: point.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

animal: animal.cpp
	$(CXX) $(CXXFLAGS) $^ -o $@

.PHONY: clean all
clean:
	rm -f point animal
