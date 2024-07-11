cmake_minimum_required(VERSION 3.16)
project(NetHoloGlyph)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Добавляем флаг -pthread для линковки с библиотекой pthread
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -pthread")

# Находим необходимые пакеты
find_package(Boost REQUIRED COMPONENTS system)
find_package(SFML 2.5 COMPONENTS graphics REQUIRED)

# Добавляем поддиректории для сервера и клиента
add_subdirectory(server)
add_subdirectory(client)
