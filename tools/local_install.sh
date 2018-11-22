#!/bin/bash

set -e
npm pack
npm install -g $(ls atcoder-*.tgz)
rm atcoder-*.tgz
