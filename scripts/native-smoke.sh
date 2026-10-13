#!/usr/bin/env bash
# Native (Blitz) smoke test: starts apps/native once per demo under Xvfb, waits, takes a
# screenshot and records whether it stayed alive and whether it logged a panic.
#
#   scripts/native-smoke.sh [--build] [--wait SECS] [--out DIR] [demo ...]
#
# Needs: dx, xvfb-run, ImageMagick (import), libxkbcommon-x11 and a Vulkan driver
# (mesa-vulkan-drivers gives the software one). Exit status is 1 if any demo failed.
#
# A demo fails if it exits, panics or draws a blank window. A demo that keeps the CPU
# above 85% for a second after start-up only gets a "busy" note: that is how a spin loop
# looks (the sidebar once did), but animated demos (spinner, shimmer, marquee) also repaint
# constantly in a debug build. Screenshots go to DIR (default target/native-smoke) for a
# human to look at: this checks that the demo runs and draws, not that it looks right.
set -uo pipefail
cd "$(dirname "$0")/.."

WAIT=8
OUT=target/native-smoke
BUILD=0
DEMOS=()
while [ $# -gt 0 ]; do
  case "$1" in
    --build) BUILD=1 ;;
    --wait) WAIT="$2"; shift ;;
    --out) OUT="$2"; shift ;;
    *) DEMOS+=("$1") ;;
  esac
  shift
done

if [ "$BUILD" = 1 ] || [ ! -x target/dx/native/debug/linux/app/native ]; then
  (cd apps/native && dx build --renderer native --platform linux) || exit 1
fi
if [ ${#DEMOS[@]} -eq 0 ]; then
  mapfile -t DEMOS < <(grep 'has_demo: true' apps/docs/src/generated.rs | grep -o 'name: "[a-z_0-9]*"' | cut -d'"' -f2)
fi

mkdir -p "$OUT"
OUT=$(cd "$OUT" && pwd)
ROOT=$PWD
fail=0
for d in "${DEMOS[@]}"; do
  log="$OUT/$d.log"
  status=$(cd target/dx/native/debug/linux/app && DEMO="$d" xvfb-run -a -s "-screen 0 1024x768x24" bash -c '
    ./native > "'"$log"'" 2>&1 &
    P=$!
    sleep '"$WAIT"'
    if kill -0 $P 2>/dev/null; then
      a=$(awk "{print \$14+\$15}" /proc/$P/stat); sleep 1; b=$(awk "{print \$14+\$15}" /proc/$P/stat)
      [ $((b - a)) -gt 85 ] && busy=" busy" || busy=""
      import -window root "'"$OUT/$d.png"'" 2>/dev/null
      kill $P
      colors=$(convert "'"$OUT/$d.png"'" -crop 800x600+0+0 -format "%k" info: 2>/dev/null || echo 0)
      if [ "${colors:-0}" -lt 2 ]; then echo blank; else echo "ok$busy"; fi
    else
      echo died
    fi')
  note=""
  case "$status" in "ok busy") note=" (busy)"; status=ok ;; esac
  if [ "$status" != ok ] || grep -q "panicked" "$log"; then
    echo "FAIL $d ($status)"; grep -m2 "panicked" -A1 "$log" | sed 's/^/     /'
    fail=1
  else
    echo "ok   $d$note"
  fi
done
exit $fail
