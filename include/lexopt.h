#include <stdbool.h>
#include <stdint.h>

typedef struct {
    enum {
        LEXOPT__STATE_NONE,
        LEXOPT__STATE_PENDING_VALUE,
        LEXOPT__STATE_SHORTS,
#ifdef _WIN32
        LEXOPT__STATE_SHORTS_U16,
#endif
        LEXOPT__STATE_FINISHED_OPS,
    } tag;
    union {
        struct {} none;
        struct { char *a; size_t a_len; } pending_value;
        struct { uint8_t *a; size_t a_len; size_t b; } shorts;
#ifdef _WIN32
        struct { uint16_t *a; size_t a_len; size_t b; } shorts_u16;
#endif
        struct {} finished_ops;
    } value;
} lexopt__state;

typedef struct {} lexopt__last_option;

/**
 * A parser for command line arguments.
 */
typedef struct {
    void *_source;
    bool (* _source_next)(void *, char const **, size_t *);
    lexopt__state _state;
    lexopt__last_option _last_option;
    char *_bin_name;
    size_t _bin_name_len;
} lexopt_parser;

size_t lexopt_parser_bin_name(lexopt_parser const *const p, char const **const out_bin_name);

lexopt_parser lexopt_parser_from_args(void *args, bool (*const args_next)(void *, char const **, size_t *));

lexopt_parser lexopt_parser_from_env();

lexopt_parser lexopt_parser_from_iter(void *args, bool (*const args_next)(void *, char const **, size_t *));

bool lexopt_parser_next(lexopt_parser *const p, lexopt_arg *const arg);

bool lexopt_parser_optional_value(lexopt_parser *const p, char const **const value, size_t *const len);

lexopt_raw_args lexopt_parser_raw_args(lexopt_parser *const p);

bool lexopt_parser_try_raw_args(lexopt_parser *const p, lexopt_raw_args *const args);


