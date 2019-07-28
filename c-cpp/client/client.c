#define _DEFAULT_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>

int client(char *host, int port)
{
        // Setup the socket
        int s = socket(AF_INET, SOCK_STREAM, 0);

        // Make the connection
        struct sockaddr_in sin;
        memset(&sin, 0, sizeof(sin));
        sin.sin_family = AF_INET;
        sin.sin_port = htons(port);
        inet_aton(host, &sin.sin_addr);

        if (connect(s, (struct sockaddr *)&sin, sizeof(sin)) != 0) {
                perror("connect");
                exit(1);
        }

        // Write the request
        char stuff[] = "GET /Makefile HTTP/1.0\r\n\r\n";
        int len = strlen(stuff);
        for (int i = 0; i < len; i++) {
                write(s, &stuff[i], 1);
                putchar(stuff[i]);
                sleep(1);
        }

        // Print the bytes that come back
        int n;
        char buf[1024];
        while((n = read(s, buf, sizeof(buf))) > 0)
                write(1, buf, n);

        close(s);
        return 0;
}

int main(int argc, char *argv[])
{
        client("127.0.0.1", 8000);
        return 0;
}
