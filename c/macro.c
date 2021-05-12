struct command {
    char *name;
    void (*function)();
};

#define COMMAND(NAME) { #NAME, NAME ## _command }

struct command commands[] = {
    COMMAND(quit),
    COMMAND(help),
};

->

sturct command commands[] = {
    { "quit", quit_command },
    { "help", help_command },
};

#define eprintf(...) fprintf(stderr, __VA_ARGS__)

eprintf("%s:%d: ", __FILE__, __LINE__);

->

fprintf(stderr, "%s:%d: ", __FILE__, __LINE__);
