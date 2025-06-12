cmake_minimum_required(VERSION 3.1.0)

# Project configuration.
project(mmscenegraph)
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 6)
set(PROJECT_VERSION_PATCH 1)
set(PROJECT_VERSION "${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}")
set(PROJECT_HOMEPAGE_URL "https://nyxspace.com/anise")
set(PROJECT_DESCRIPTION "C++ bindings to ANISE, a modern SPICE rewrite")
set(PROJECT_AUTHOR "Christopher Rabotin")
set(PROJECT_COPYRIGHT "2025, Christopher Rabotin")


# Static build information.
set(target_lib "anise")
set(target_test_exe "anise_cpptests")


# Add custom modules to the module path
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${CMAKE_CURRENT_SOURCE_DIR}/cmake_modules)
include(CxxDemoExampleUtils)

set_cxx_standard_cpp_11()

add_subdirectory(src)
add_subdirectory(include)
add_subdirectory(tests)