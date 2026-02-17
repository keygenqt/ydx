#include "napi/native_api.h"

extern "C" {
    // Import from Rust (defined in libshared.a)
    int shared_performance_test();

    // Wrapper and logic in a single method
    static napi_value CallTest(napi_env env, napi_callback_info) {
        napi_value res;
        napi_create_int32(env, shared_performance_test(), &res);
        return res;
    }

    // Module registration
    static napi_value Init(napi_env env, napi_value exports) {
        napi_property_descriptor desc[] = {
            { "shared_performance_test", nullptr, CallTest, nullptr, nullptr, nullptr, napi_default, nullptr }
        };
        napi_define_properties(env, exports, sizeof(desc) / sizeof(desc), desc);
        return exports;
    }

    // Module definition (nm_modname must match the target name in CMakeLists.txt)
    napi_module _module = { .nm_version = 1, .nm_register_func = Init, .nm_modname = "shared" };

    // Auto-registration constructor
    __attribute__((constructor)) void Register() { napi_module_register(&_module); }
}
