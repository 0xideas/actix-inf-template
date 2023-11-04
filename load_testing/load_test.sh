#!/bin/bash

timestamp_one=$(date +%s)
for i in {1..1000}
do
   curl -X POST -H "Content-Type: application/json" -d '{"data": [-5.0, 2.0, 0.0]}' http://127.0.0.1:8080/api/infer &
done
wait
timestamp_two=$(date +%s)

diff=$((timestamp_two-timestamp_one))
echo ""
echo "${diff}"
echo ""