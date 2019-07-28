CC := gcc
CFLAGS := -Wall -g

a.out: client.c
	$(CC) $(CFLAGS) $^ -o $@

.PHONY: clean
clean:
	rm -f a.out
