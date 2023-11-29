#!/bin/bash

# URL="http://127.0.0.1:8000/check_health"
# URL="http://127.0.0.1:8000/subscriptions"
URL="https://rusty-krab-xkbai.ondigitalocean.app/subscriptions"

echo
echo "Sending request: ${URL}..."
curl ${URL} \
  -H "Content-Type: application/x-www-form-urlencoded" \
  --data-urlencode "name=Brian Fong" \
  --data-urlencode "email=0xfrian@gmail.com" \
  --verbose
