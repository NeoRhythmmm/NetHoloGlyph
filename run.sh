#!/bin/bash

# Создание директории для сборки
mkdir -p build
cd build

# Генерация Makefile с помощью CMake
cmake ..

# Компиляция проекта
make

# Проверка содержимого каталога сборки
echo "Содержимое каталога сборки:"
ls -R

# Проверка наличия файлов сервера и клиента
if [ -f ./server/server ]; then
    echo "Сервер найден."
else
    echo "Сервер не найден."
fi

if [ -f ./client/client ]; then
    echo "Клиент найден."
else
    echo "Клиент не найден."
fi

# Запуск сервера в фоновом режиме
if [ -f ./server/server ]; then
    ./server/server &
else
    echo "Ошибка: Серверный файл не найден."
    exit 1
fi

# Запуск клиента
if [ -f ./client/client ]; then
    ./client/client
else
    echo "Ошибка: Клиентский файл не найден."
    exit 1
fi
