#~/usr/bin.bash

CONTEST_NAME="$1"
echo "Let's start $CONTEST_NAME!"

cargo compete new $CONTEST_NAME

cd $CONTEST_NAME

for f in ./src/bin/*; do
    open $f
done

# cargo compete open
for f in ./src/bin/*; do
    echo "open https://atcoder.jp/contests/$CONTEST_NAME/tasks/$CONTEST_NAME"_"`basename ${f} .rs`";
done | tail -r