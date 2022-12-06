cmake_minimum_required(VERSION 3.1)
project(day_05)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_SOURCE_DIR})

set(SOURCES main.cpp)

add_executable(${PROJECT_NAME} ${SOURCES})
