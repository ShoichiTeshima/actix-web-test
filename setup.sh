#!/usr/bin/env bash

set -euo pipefail

#何すんだっけ

sleep 10
#db setup
exec diesel setup
exec diesel migration run

exec cargo run --bin actix-web-test 