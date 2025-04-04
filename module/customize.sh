#!/system/bin/sh
echo "Hello World!"
if [ ! -f /storage/emulated/0/Android/cosa_apps.toml ]; then
    cp $MODPATH/cosa_apps.toml /storage/emulated/0/Android/cosa_apps.toml
fi

time=$(date "+%Y-%m-%d_%H:%M:%S")
cp -af /storage/emulated/0/Android/cosa_apps.toml /storage/emulated/0/Android/thread_opt_"$time"backup.toml
cp -f $MODPATH/cosa_apps.toml /storage/emulated/0/Android/cosa_apps.toml

echo "仓库地址: https://github.com/reigadegr/cosa_helper"
