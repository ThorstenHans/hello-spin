#! /bin/bash

echo " - Launching Message Publisher"

cd message-publisher
spin build --up &
cd ..

echo " - Launching Message Consumer (RUST)"

cd message-consumer
spin build --up &
cd ..

echo " - Launching Message Consumer (GoLang)"

cd message-consumer-go
spin build --up &
cd ..

echo "🚀 ervything is up and running!"
wait
