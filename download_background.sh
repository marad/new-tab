#!/bin/bash

TARGET="static/background.jpg"
URL=$(curl -s https://api.reddit.com/r/earthporn/random -L -H'User-Agent: linux:io.github.marad.newtab:v0.0.1' | jq '.[0].data.children[0].data.url' -r)

echo "Downloading $URL"
curl -s "$URL" --output "$TARGET"

