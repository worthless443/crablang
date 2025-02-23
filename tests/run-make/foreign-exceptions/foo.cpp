#include <assert.h>
#include <stddef.h>
#include <stdio.h>

void println(const char* s) {
    puts(s);
    fflush(stdout);
}

struct exception {};
struct crablang_panic {};

struct drop_check {
    bool* ok;
    ~drop_check() {
        println("~drop_check");

        if (ok)
            *ok = true;
    }
};

extern "C" {
    void crablang_catch_callback(void (*cb)(), bool* crablang_ok);

    void throw_cxx_exception() {
        println("throwing C++ exception");
        throw exception();
    }

    void test_cxx_exception() {
        bool crablang_ok = false;
        try {
            crablang_catch_callback(throw_cxx_exception, &crablang_ok);
            assert(false && "unreachable");
        } catch (exception e) {
            println("caught C++ exception");
            assert(crablang_ok);
            return;
        }
        assert(false && "did not catch thrown C++ exception");
    }

    void cxx_catch_callback(void (*cb)(), bool* cxx_ok) {
        drop_check x;
        x.ok = NULL;
        try {
            cb();
        } catch (crablang_panic e) {
            assert(false && "shouldn't be able to catch a crablang panic");
        } catch (...) {
            println("caught foreign exception in catch (...)");
            // Foreign exceptions are caught by catch (...). We only set the ok
            // flag if we successfully caught the panic. The destructor of
            // drop_check will then set the flag to true if it is executed.
            x.ok = cxx_ok;
            throw;
        }
    }
}
