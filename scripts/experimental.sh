#!/bin/bash

# URL="http://127.0.0.1:8000/check_health"
URL="http://127.0.0.1:8000/subscriptions"

echo
echo "Sending request: ${URL}..."
curl -s ${URL} \
  -X POST \
  -H "Content-Type: application/x-www-form-urlencoded" \
  --data-urlencode "name=Brian Fong" \
  --data-urlencode "email=0xfrian@gmail.com" \
