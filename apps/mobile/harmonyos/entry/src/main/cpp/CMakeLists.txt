cmake_minimum_required(VERSION 3.10)
project(MyHarmonyProject)

# Define the path to the directory containing the compiled Rust library
set(CORE_LIB_DIR "../../../../../../../core/build/harmonyos/aarch64-unknown-linux-ohos")
# Convert the relative path to an absolute path for reliability
get_filename_component(CORE_LIB_DIR_ABS ${CORE_LIB_DIR} ABSOLUTE)

# Include directory for C/C++ headers (to find libcore.h)
include_directories(${CORE_LIB_DIR_ABS})

# Declare the pre-compiled Rust static library as an IMPORTED target
add_library(libydx_harmonyos_static STATIC IMPORTED)
# Specify the physical location of the .a file for the imported target
set_target_properties(libydx_harmonyos_static PROPERTIES
    IMPORTED_LOCATION ${CORE_LIB_DIR_ABS}/libydx_harmonyos.a)

# Define the main core library (the NAPI module) and its source file
# This will produce libydx_harmonyos.so
add_library(ydx_harmonyos SHARED ydx_harmonyos.cpp)

# Link the NAPI module with the Rust static library and system dependencies
target_link_libraries(ydx_harmonyos
    PUBLIC
    libydx_harmonyos_static  # Our Rust logic
    libace_napi.z.so         # Essential for Node-API functions (napi_*)
    libhilog_ndk.z.so        # Required for logging (HiLog)
)
