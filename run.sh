#!/bin/bash
for i in {1..19}; do
target/release/hypr ./hypr-protect/$i &
done