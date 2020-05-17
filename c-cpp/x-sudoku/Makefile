NAME := sudoku

CXX := g++
CXXFLAGS := -Wall -std=c++17

ifneq ($(RELEASE),1)
all: $(NAME)
run: $(NAME)
	@printf >&2 "     \e[32;1mRunning\e[37;0m \`./$(NAME)\`\n"
	@./$(NAME)
else
all: $(NAME)_rel
run: $(NAME)_rel
	@printf >&2 "     \e[32;1mRunning\e[37;0m \`./$(NAME)_rel\`\n"
	@./$(NAME)_rel
endif

$(NAME): CXXFLAGS += -g -fsanitize=unreachable
$(NAME): $(NAME).cpp
	@printf >&2 "   \e[32;1mCompiling\e[37;0m $(NAME) [debug]\n"
	@$(CXX) $(CXXFLAGS) $^ -o $@

$(NAME)_rel: CXXFLAGS += -g -O3
$(NAME)_rel: $(NAME).cpp
	@printf >&2 "   \e[32;1mCompiling\e[37;0m $(NAME) [release]\n"
	@$(CXX) $(CXXFLAGS) $^ -o $@

clean:
	$(RM) $(NAME) $(NAME)_rel

.PHONY: all clean run
