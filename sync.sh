if [ "$#" -ne 2 ]; then
    echo "Usage: $0 [release/debug] [RPI IP]"
    exit 1
fi

if [ "$1" != "release" ] && [ "$1" != "debug" ]; then
    echo "Usage: $0 [release/debug] [RPI IP]"
    echo "release, debug"
    exit 1
fi

scp target/arm-unknown-linux-gnueabihf/$1/avcproject-team-28 pi@$2:~