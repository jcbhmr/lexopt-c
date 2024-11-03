#include <lexopt.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <ctype.h>

typedef struct args {
    const char *thing;
    uint32_t number;
    bool shout;
} args;

const char* parse_args(args *const args_v) {
    const char* thing = nullptr;
    uint32_t number = 1;
    bool shout = false;
    lexopt_parser parser = lexopt_parser_from_env();
    while (true) {
        lexopt_arg arg;
        bool ok;
        const char* err = lexopt_parser_next(parser, &arg, &ok);
        if (err) {
            return err;
        }
        if (!ok) {
            break;
        }
        if ((arg.tag == lexopt_arg_Tag_short && arg.short_._1 == 'n') || (arg.tag == lexopt_arg_Tag_long && strncmp(arg.long_.name, "number", 6) == 0)) {
            const char* number_text;
            err = lexopt_parser_value(parser, &number_text);
            if (err) {
                return err;
            }
            char* end;
            number = strtoul(number_text, &end, 10);
            if (*end) {
                return "invalid number";
            }
        } else if (arg.tag == lexopt_arg_Tag_long && strncmp(arg.long_.name, "shout", 5) == 0) {
            shout = true;
        } else if (arg.tag == lexopt_arg_Tag_value && !thing) {
            size_t a_len = strlen(arg.value.a);
            thing = malloc(a_len + 1);
            if (!thing) {
                return "out of memory";
            }
            strncopy(thing, arg.value.a, a_len);
            thing[a_len] = '\0';
        } else if (arg.tag == lexopt_arg_Tag_long && strncmp(arg.long_.name, "help", 4) == 0) {
            puts("Usage: hello [-n|--number=NUM] [--shout] THING");
            exit(0);
        } else {
            return lexopt_arg_unexpected(arg);
        }
    }

    if (!thing) {
        return "missing argument THING";
    }
    *args_v = args{
        .thing = thing,
        .number = number,
        .shout = shout,
    };
    return nullptr;
}

int main() {
    args args_v;
    const char* err = parse_args(&args_v);
    if (err) {
        fputs(err, stderr);
        return 1;
    }
    size_t message_cap = strlen(args_v.thing) + 8;
    char* message = malloc(message_cap);
    if (!message) {
        fputs("out of memory", stderr);
        return 1;
    }
    int errnum = snprintf(message, message_cap, "Hello %s!", args_v.thing);
    if (errnum < 0) {
        fputs("snprintf failed", stderr);
        return 1;
    } else if (errnum >= message_cap) {
        fputs("message too long", stderr);
        return 1;
    }
    if (args_v.shout) {
        size_t message_len = strlen(message);
        for (size_t i = 0; i < message_len; i++) {
            message[i] = toupper(message[i]);
        }
    }
    for (uint32_t i = 0; i < args_v.number; i++) {
        puts(message);
    }
    return 0;
}