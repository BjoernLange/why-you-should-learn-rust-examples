#!/bin/python3

from pathlib import Path
import os

if __name__ == '__main__':
    target = 'target'
    Path(target).mkdir(parents=True, exist_ok=True)
    main = os.path.join(target, 'main')
    exit(os.system(f'g++ -o {main} main.cc'))
