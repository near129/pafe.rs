#!/bin/bash
set -e

cd libpafe

rm -f config.guess config.sub
curl -o config.guess 'http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.guess'
curl -o config.sub 'http://git.savannah.gnu.org/gitweb/?p=config.git;a=blob_plain;f=config.sub'

if [ -f configure.in ]; then
    mv configure.in configure.ac
fi

aclocal
autoconf
autoheader
automake --add-missing
libtoolize
