#!/usr/bin/env python3

import sys
import csv

def main():
    if len(sys.argv) < 2:
        print('Usage: entry_generator.py file.csv')
        sys.exit(1)
    with open(sys.argv[1], 'r') as fis:
        reader = csv.reader(fis, delimiter='|')
        s = True
        for row in reader:
            if s:
                s = False
                continue
            i = int(row[2])
            offer = []
            if i & 1:
                offer.append('Fall')
            if i & 2:
                offer.append('Spring')
            if i & 4:
                offer.append('Summer')
            offer = ' and '.join(offer)
            entry = (f'{row[0]} has the course title "{row[1]}".\n'
                     f'{row[0]} is a {row[3]} credit course taught by {row[4]}.\n'
                     f'{row[0]} is offered every {offer}.\n'
                     f'{row[5]}\n')
            with open('../data/db/'+row[0], 'w') as fos:
                fos.write(entry)


if __name__ == '__main__':
    main()
