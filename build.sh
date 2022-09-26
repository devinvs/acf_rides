#!/bin/sh
docker build -t acf .
id=$(docker create acf)
docker cp $id:/target/release/rides ./
docker rm $id
