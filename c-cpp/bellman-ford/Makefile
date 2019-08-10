SHELL := /bin/bash
PROJECT := bellman-ford
CXX := g++
CXXFLAGS := -Wall -std=c++17
MAIN := main
DEBUG_LIB := graph bf
TEST_COMPONENT := graph bf

DEBUG_DIR := target/debug/
TEST_DIR := target/test/
CATCH_O := $(TEST_DIR)catch-main-compiled.o

# https://stackoverflow.com/a/14061796/10974106
ifeq (test,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif
ifeq (run,$(firstword $(MAKECMDGOALS)))
  RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
  $(eval $(RUN_ARGS):;@:)
endif

all:
	@echo "Available subcommands:"
	@echo "    test"
	@echo "    build"
	@echo "    run"
	@echo "    clean"

$(TEST_DIR)%.o: %.cpp | $(TEST_DIR)
	$(CXX) -c $(CXXFLAGS) -o $@ $<

$(DEBUG_DIR)%.o: %.cpp graph.h | $(DEBUG_DIR)
	$(CXX) -c $(CXXFLAGS) -o $@ $<

$(CATCH_O): | $(TEST_DIR)
	$(CXX) -x c++ -c $(CXXFLAGS) -o $@ \
		<(echo -e "#define CATCH_CONFIG_MAIN\n#include <catch2/catch.hpp>")

$(TEST_DIR):
	mkdir -p $@

$(DEBUG_DIR):
	mkdir -p $@

$(TEST_DIR)$(PROJECT): $(CATCH_O) $(TEST_COMPONENT:%=$(TEST_DIR)%.o) | $(TEST_DIR)
	$(CXX) $(CXXFLAGS) -o $@ $^

$(DEBUG_DIR)$(PROJECT): CXXFLAGS += -DCATCH_CONFIG_DISABLE -g
$(DEBUG_DIR)$(PROJECT): $(DEBUG_LIB:%=$(DEBUG_DIR)%.o) $(DEBUG_DIR)$(MAIN).o | $(DEBUG_DIR)
	$(CXX) $(CXXFLAGS) -o $@ $^

build: $(DEBUG_DIR)$(PROJECT)

run: build
	@echo -e "\e[32;1m  Running:\e[m $(DEBUG_DIR)$(PROJECT)"
	@$(DEBUG_DIR)$(PROJECT) $(RUN_ARGS)

test: $(TEST_DIR)$(PROJECT)
	@echo -e "\e[32;1m  Running:\e[m $<"
	@$< $(RUN_ARGS)

clean:
	$(RM) -r target

.PHONY: clean test run build all
