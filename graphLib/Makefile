################################################################################
#	File:           Makefile for Library graph and program tao
#   	Author:         Marion Bohr
#   	Creation Date:  08/21/2024
#   	Description:    Makefile 
#   			For building graph and tao
#   	Build:          make -f Makefile
################################################################################

# Compiler and flags
CXX := g++
CXXFLAGS := -Wall -Wextra -pedantic -std=c++17 -O2

# Target executable
TARGET := tao

# Source files and objects
SOURCES := tao.cpp graph.cpp
OBJECTS := $(SOURCES:.cpp=.o)

# Header files
HEADERS := graph.h

# Default rule: build the executable
all: $(TARGET)

# Link the executable
$(TARGET): $(OBJECTS)
	$(CXX) $(CXXFLAGS) -o $@ $(OBJECTS)

# Compile source files into object files
%.o: %.cpp $(HEADERS)
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean rule to remove build artifacts
clean:
	rm -f $(OBJECTS) $(TARGET)

# Phony targets (targets that do not correspond to files)
.PHONY: all clean

