#include "kernel/types.h"
#include "user/user.h"

const char* PING_MESSAGE = "ping";
const char* PONG_MESSAGE = "pong";


int main(int argc, char **argv) {
    //TODO: add second pipe
    int pipe_fd[2];
    int pipe_fd1[2];
    char buffer[100];

    if (pipe(pipe_fd) == -1 || pipe(pipe_fd1) == -1) {
        fprintf(2, "Error creating pipe");
        exit(1);
    }

    int child_pid = fork();

    if (child_pid == -1) {
        fprintf(2, "Error on fork");
        exit(1);
    }

    if (child_pid == 0) {
        //Child process
        if (read(pipe_fd[0], buffer, sizeof(buffer)) == -1) {
            fprintf(2, "Error on read PING");
            exit(1);
        }

        fprintf(1, "%d: got \'%s\'\n", getpid(), buffer);
        if (write(pipe_fd[1], PONG_MESSAGE, strlen(PONG_MESSAGE) + 1) == -1) {
            fprintf(2, "Error on write PONG");
            exit(1);
        }
        close(pipe_fd[0]);
        close(pipe_fd[1]);

        exit(0);
    }else {
        //Parent process
        if (write(pipe_fd[1], PING_MESSAGE, strlen(PING_MESSAGE) + 1) == -1) {
            fprintf(2, "Error on write PING");
            exit(1);
        }

        wait(0);

        if (read(pipe_fd[0], buffer, sizeof(buffer)) == -1) {
            fprintf(2, "Error on read PONG");
            exit(1);
        }
        fprintf(1, "%d: got \'%s\'\n", getpid(), buffer);


        close(pipe_fd[0]);
        close(pipe_fd[1]);
        exit(0);
    }
}