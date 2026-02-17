cmake_minimum_required(VERSION 3.10)
project(MyHarmonyProject)

# Define the path to the directory containing the compiled Rust library
set(SHARED_LIB_DIR "../../../../../../shared/rust/libs/ohosArm64")
# Convert the relative path to an absolute path for reliability
get_filename_component(SHARED_LIB_DIR_ABS ${SHARED_LIB_DIR} ABSOLUTE)

# Include directory for C/C++ headers (to find libshared_api.h)
include_directories(${SHARED_LIB_DIR_ABS})

# Declare the pre-compiled Rust static library as an IMPORTED target
add_library(shared_static STATIC IMPORTED)
# Specify the physical location of the .a file for the imported target
set_target_properties(shared_static PROPERTIES
    IMPORTED_LOCATION ${SHARED_LIB_DIR_ABS}/libshared.a)

# Define the main shared library (the NAPI module) and its source file
# This will produce libshared.so
add_library(shared SHARED shared.cpp)

# Link the NAPI module with the Rust static library and system dependencies
target_link_libraries(shared
    PUBLIC
    shared_static       # Our Rust logic
    libace_napi.z.so    # Essential for Node-API functions (napi_*)
    libhilog_ndk.z.so   # Required for logging (HiLog)
)
