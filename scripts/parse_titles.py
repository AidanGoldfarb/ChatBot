#!/usr/bin/env python3

import sys
import csv

def main():
    if len(sys.argv) < 2:
        print('Usage: parse_titles.py file.csv')
        sys.exit(1)
    with open(sys.argv[1], 'r') as fis:
        with open('../data/db/titles', 'w') as fos:
            reader = csv.reader(fis, delimiter='|')
            s = True
            for row in reader:
                if s:
                    s = False
                    continue
                fos.write(f'{row[0]}|{row[1]}\n')


if __name__ == '__main__':
    main()
