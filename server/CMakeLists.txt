cmake_minimum_required(VERSION 3.16)
project(AudioServer)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Добавляем исходный код сервера
add_executable(server server.cpp)

# Линкуем с Boost и pthread
target_link_libraries(server ${Boost_LIBRARIES} pthread)
