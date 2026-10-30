#!/usr/bin/env bash
# Install an APK on the running emulator, launch it, and fail if it crashes or panics.
# Used by .github/workflows/android.yml.
set -euo pipefail
apk="$1"
aapt="$(ls "$ANDROID_HOME"/build-tools/*/aapt | sort -V | tail -1)"
package="$("$aapt" dump badging "$apk" | sed -n "s/^package: name='\([^']*\)'.*/\1/p")"
activity="$("$aapt" dump badging "$apk" | sed -n "s/^launchable-activity: name='\([^']*\)'.*/\1/p")"
echo "package $package, activity $activity"

adb install -r "$apk"
adb logcat -c
adb shell am start -W -n "$package/$activity"

# Give the webview time to load the app, then check it is still running.
for _ in $(seq 1 30); do
  sleep 2
  if adb logcat -d | grep -qE "panicked at|FATAL EXCEPTION"; then
    break
  fi
done
adb logcat -d > logcat.txt || true
if grep -nE "panicked at|FATAL EXCEPTION" logcat.txt; then
  echo "the app crashed"; tail -200 logcat.txt; exit 1
fi
if ! adb shell pidof "$package" >/dev/null; then
  echo "the app is not running"; tail -200 logcat.txt; exit 1
fi
adb shell screencap -p /sdcard/gallery.png && adb pull /sdcard/gallery.png . || true
echo "the gallery started and kept running"
