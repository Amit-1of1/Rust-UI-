#!/usr/bin/env bash
# Install an APK on the running emulator, launch it, and fail if it crashes or panics.
# Used by .github/workflows/android.yml.
set -euo pipefail
apk="$1"
aapt="$(ls "$ANDROID_HOME"/build-tools/*/aapt | sort -V | tail -1)"
package="$("$aapt" dump badging "$apk" | sed -n "s/^package: name='\([^']*\)'.*/\1/p")"
activity="$("$aapt" dump badging "$apk" | sed -n "s/^launchable-activity: name='\([^']*\)'.*/\1/p")"
echo "package $package, activity $activity"

# Let the freshly booted emulator settle first (boot animation done, keyguard gone).
for _ in $(seq 1 30); do
  [ "$(adb shell getprop init.svc.bootanim | tr -d '\r')" = stopped ] && break
  sleep 2
done
adb shell wm dismiss-keyguard || true
sleep 15

adb install -r "$apk"
adb logcat -c
adb shell am start -W -n "$package/$activity"

# Give the webview time to load the app; stop early if the process goes away.
pid="$(adb shell pidof "$package" || true)"
echo "pid ${pid:-none} after launch"
alive=yes
for _ in $(seq 1 30); do
  sleep 2
  if ! adb shell pidof "$package" >/dev/null; then
    alive=no
    break
  fi
done
adb logcat -d > logcat.txt || true
# What the app and the system said about it: its own process, crashes, kills.
grep -nE "${pid:-^$}|$package|Fatal signal|DEBUG +:|AndroidRuntime|RustStdoutStderr|lmkd|lowmemorykiller|am_kill|Process .* has died|dioxus|[Cc]onfig changes|[Rr]elaunch|onConfigurationChanged" logcat.txt \
  | tail -150 > app-log.txt || true
cat app-log.txt
if grep -nE "panicked at|FATAL EXCEPTION|Fatal signal" logcat.txt; then
  echo "the app crashed"; exit 1
fi
if [ "$alive" = no ]; then
  echo "the app stopped running"; exit 1
fi
adb shell screencap -p /sdcard/gallery.png && adb pull /sdcard/gallery.png . || true
echo "the gallery started and kept running for a minute"
