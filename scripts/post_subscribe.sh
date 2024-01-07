#!/bin/bash

curl -X POST http://localhost:8080/subscribe \
     -H 'Content-Type: application/x-www-form-urlencoded' \
     -d 'name=John%20Doe&email=john.doe%40example.com'