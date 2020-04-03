CFLAGS := -Wall -std=c99
LDFLAGS := -lcrypto

a.out: encfile.c
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

clean:
	$(RM) a.out

.PHONY: clean
