SHELL := /bin/bash
CXX := g++
CXXFLAGS := -Wall -std=c++17 -g
DEBUG_BIN := main
DEBUG_LIB := graph
TEST_BIN := $(DEBUG_LIB)

DEBUG_DIR := target/debug
TEST_DIR := target/test
CATCH_O := $(TEST_DIR)/catch-main-compiled.o

# https://stackoverflow.com/a/14061796/10974106
ifeq (test,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif
ifeq (run,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
  CXXFLAGS := $(CXXFLAGS) -DCATCH_CONFIG_DISABLE
endif
ifeq (build,$(firstword $(MAKECMDGOALS)))
  CXXFLAGS := $(CXXFLAGS) -DCATCH_CONFIG_DISABLE
endif

all:
	@echo "Available subcommands:"
	@echo "    test"
	@echo "    build"
	@echo "    run"
	@echo "    clean"

$(TEST_DIR)/%.o: %.cpp | $(TEST_DIR)
	$(CXX) -c $(CXXFLAGS) -o $@ $^

$(DEBUG_DIR)/%.o: %.cpp | $(DEBUG_DIR)
	$(CXX) -c $(CXXFLAGS) -o $@ $^

$(CATCH_O): | $(TEST_DIR)
	$(CXX) -x c++ -c $(CXXFLAGS) -o $@ \
		<(echo -e "#define CATCH_CONFIG_MAIN\n#include <catch2/catch.hpp>\n")

$(TEST_DIR):
	mkdir -p $@

$(DEBUG_DIR):
	mkdir -p $@

$(TEST_DIR)/$(TEST_BIN): $(CATCH_O) $(TEST_DIR)/$(TEST_BIN).o | $(TEST_DIR)
	$(CXX) $(CXXFLAGS) -o $@ $^

$(DEBUG_DIR)/$(DEBUG_BIN): $(DEBUG_DIR)/$(DEBUG_LIB).o $(DEBUG_DIR)/$(DEBUG_BIN).o | $(DEBUG_DIR)
	$(CXX) $(CXXFLAGS) -o $@ $^

build: $(DEBUG_DIR)/$(DEBUG_BIN)

run: build
	@echo -e "\e[32;1m  Running:\e[m $(DEBUG_DIR)/$(DEBUG_BIN)"
	@$(DEBUG_DIR)/$(DEBUG_BIN) $(RUN_ARGS)

test: $(TEST_DIR)/$(TEST_BIN)
	@echo -e "\e[32;1m  Running:\e[m $<"
	@$< $(RUN_ARGS)

clean:
	$(RM) -r target

.PHONY: clean test run build all
