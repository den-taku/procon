#~/usr/bin.bash

CONTEST_NAME="$1"
echo "Let's start $CONTEST_NAME!"

cargo compete new $CONTEST_NAME

cd $CONTEST_NAME

cargo compete open

for f in ./src/bin/*; do
    open $f
done

open src/bin/a.rs
open src/bin/b.rs