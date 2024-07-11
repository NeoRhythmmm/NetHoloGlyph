cmake_minimum_required(VERSION 3.16)
project(AudioClient)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Добавляем исходный код клиента
add_executable(client client.cpp)

# Линкуем с Boost, SFML и pthread
target_link_libraries(client ${Boost_LIBRARIES} sfml-graphics sfml-window sfml-system pthread)
