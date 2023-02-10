cmake_minimum_required(VERSION 3.9.1)
project(TinkerForgeTest C)

find_package(tinkerforge-bindings REQUIRED)

add_executable(tinkerforgePiano piano.c)
target_link_libraries(tinkerforgePiano tinkerforge::bindings)
