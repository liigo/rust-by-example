mkdir -p bin
mkdir -p stage/node_modules
ln -sf ../book.json stage

#ln -sf ../examples/README.md stage
lang=$1
if [ -n "${lang}" ] ; then
    cp examples/README_${lang}.md stage/README.md
else
    cp examples/README.md stage/README.md
fi

