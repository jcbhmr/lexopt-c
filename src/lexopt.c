#include <lexopt.h>

extern void __assert(const char *assertion, const char *file, int line);
#define rassert(expr) ((expr) ? (void)0 : __assert(#expr, __FILE__, __LINE__))

typedef struct {} inner_iter;

static make_iter(void *iter, bool (*iter_next)(void *, char const **, size_t *)) {
    
}

size_t lexopt_parser_bin_name(lexopt_parser const *const p, char const **const out_bin_name) {
    rassert(p != NULL);
    rassert(out_bin_name != NULL);
    if (p->_bin_name == NULL) {
        *out_bin_name = NULL;
        return 0;
    } else {
        *out_bin_name = p->_bin_name;
        return p->_bin_name_len;
    }
}

lexopt_parser lexopt_parser_from_args(void *args, bool (*const args_next)(void *, char const **, size_t *)) {

}
