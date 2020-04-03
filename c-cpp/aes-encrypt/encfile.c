#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <unistd.h>

#include <openssl/aes.h>

void encrypt(FILE *fp, const char *passwd, int pwlen)
{
        unsigned char keybuf[32];
        memset(keybuf, 0x90, 32);
        memcpy(keybuf, passwd, sizeof(int) * (pwlen < 32 ? pwlen : 32));

        // Setup key
        AES_KEY aeskey;
        AES_set_encrypt_key(keybuf, 256, &aeskey);

        // Encrypt file and print to stdout
        unsigned char buf[16];
        unsigned char ebuf[16];
        bool done = false;
        while (!feof(fp) && !done) {
                memset(buf, 0, 16);
                size_t n = fread(buf, sizeof(char), 16, fp);
                if (n < 16) {
                        // The last block. Fill the buffer with the number of
                        // bytes needed to fill the 16 bytes buffer.
                        memset(buf + n, 16 - n, 16 - n);
                        done = true;
                }
                AES_encrypt(buf, ebuf, &aeskey);
                fwrite(ebuf, sizeof(char), 16, stdout);
        }
}

void decrypt(FILE *fp, const char *passwd, int pwlen)
{
        unsigned char keybuf[32];
        memset(keybuf, 0x90, 32);
        memcpy(keybuf, passwd, sizeof(int) * (pwlen < 32 ? pwlen : 32));

        // Setup key
        AES_KEY aeskey;
        AES_set_decrypt_key(keybuf, 256, &aeskey);

        // Decrypt file and print to stdout
        unsigned char buf[16];
        unsigned char pre[16];
        unsigned char dbuf[16];
        if (fread(buf, sizeof(char), 16, fp) < 16) {
                fprintf(stderr, "corrupted\n");
                exit(1);
        }
        while (!feof(fp)) {
                memcpy(pre, buf, 16);
                memset(buf, 0, 16);
                size_t n = fread(buf, sizeof(char), 16, fp);
                AES_decrypt(pre, dbuf, &aeskey);
                if (n == 0) {
                        // The last block. The number of meaningful bytes in the
                        // last block is (16 - dbuf[15]). If it's 16, that means
                        // the size of the original file is a multiple of 16 and
                        // this last block should be discarded.
                        int rem = (16 - dbuf[15]) % 16;
                        fwrite(dbuf, sizeof(char), rem, stdout);
                        break;
                }
                fwrite(dbuf, sizeof(char), 16, stdout);
        }
}

int main(int argc, char *argv[])
{
        if (argc < 4) {
                fprintf(stderr, "Usage: %s [enc|dec] file password\n", argv[0]);
                exit(1);
        }

        // Get input file, use stdin if it's "-"
        FILE *fp = strcmp(argv[2], "-") == 0 ? stdin : fopen(argv[2], "r");
        if (fp == NULL) {
                printf("fopen failed\n");
                exit(1);
        }
        char *passwd = argv[3];

        if (strcmp(argv[1], "enc") == 0) {
                // encrypt
                if (isatty(STDOUT_FILENO)) {
                        printf("Not printing into terminal\n");
                        exit(1);
                }
                encrypt(fp, passwd, strlen(passwd));
        } else if (strcmp(argv[1], "dec") == 0) {
                // decrypt
                decrypt(fp, passwd, strlen(passwd));
        }

        return 0;
}
