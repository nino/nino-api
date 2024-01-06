cmake_minimum_required(VERSION 3.22.0)
project(nino_api_cpp)
add_compile_options(-std=c++20)
add_compile_options(-Wall)
add_compile_options(-Wextra)
add_compile_options(-Wpedantic)
add_compile_options(-Werror)
add_compile_options(-Wno-unused-parameter)
add_compile_options(-Wno-unused-variable)
add_compile_options(-Wno-unused-function)
add_compile_options(-Wno-unused-but-set-variable)
add_compile_options(-Wno-unused-but-set-parameter)
add_compile_options(-Wno-missing-field-initializers)
add_compile_options(-Wno-missing-braces)
add_compile_options(-Wno-unknown-pragmas)
add_compile_options(-Wno-strict-aliasing)
add_compile_options(-Wno-implicit-fallthrough)
add_compile_options(-Wno-sign-compare)
add_compile_options(-Wno-implicit-int-float-conversion)
add_compile_options(-Wno-implicit-float-conversion)
add_compile_options(-Wno-enum-compare-switch)
add_compile_options(-Wno-enum-compare)
add_compile_options(-Wno-enum-compare-switch)

include(FetchContent)

FetchContent_Declare(
  pugixml
  GIT_REPOSITORY https://github.com/zeux/pugixml
  GIT_TAG        master # It's usually better to pin a specific commit or version tag
)

FetchContent_Declare(
  crow
  GIT_REPOSITORY https://github.com/CrowCpp/crow
  GIT_TAG        master # It's usually better to pin a specific commit or version tag
)

FetchContent_MakeAvailable(crow)
FetchContent_MakeAvailable(pugixml)

find_package(OpenSSL REQUIRED)

add_executable(api source/main.cpp source/last_year.cpp source/http.cpp)

target_link_libraries(api PRIVATE pugixml Crow::Crow)
target_include_directories(api PRIVATE ${OPENSSL_INCLUDE_DIR})
target_link_libraries(api PRIVATE OpenSSL::SSL OpenSSL::Crypto)
