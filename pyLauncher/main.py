#!/usr/bin/env python3
import subprocess
import sys
import json

from pathlib import Path
from os.path import isdir, isfile, join, basename, dirname, realpath


if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f'Usage: {sys.argv[0]} <file_path>')
        sys.exit(1)
    file_path = sys.argv[1]
    assert isfile(file_path)
    executable = join(Path(realpath(__file__)).parent.parent, 'target', 'release', 'ngramsxtc')
    assert isfile(executable)

    jout = None
    try:
        jout = json.loads(subprocess.check_output([executable, file_path], stderr=subprocess.STDOUT))
    except Exception as e:
        sys.exit(str(e))
    
    for window, ngrams in jout.items():
        print(f'#{window}={len(ngrams)}')
        wsize = int(window)
        for ngram in ngrams:
            assert len(ngram) == wsize
