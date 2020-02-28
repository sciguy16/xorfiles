#!/usr/bin/env python

import sys

if len(sys.argv) != 3:
    print("Usage: ./xorfiles.py FILE1 FILE2")
    sys.exit()

_, file1, file2, *_ = sys.argv

BUFSIZE = 4096

with open(file1, 'rb') as f1:
    with open(file2, 'rb') as f2:
        while True:
            buf1 = f1.read(BUFSIZE)
            buf2 = f2.read(BUFSIZE)
            # When .read() runs out of file it returns an empty
            # string instead of, you know, erroring or otherwise
            # indicating that the file has run out -.-
            if buf1 == b'' or buf2 == b'':
                break
            print(''.join(map(
                lambda x: chr(x[0] ^ x[1]),
                zip(buf1, buf2)
                )), end='')
print("")
