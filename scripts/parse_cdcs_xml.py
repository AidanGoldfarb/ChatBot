#!/usr/bin/env python3

# Parses the XML returned from
# https://cdcs.ur.rochester.edu/XMLQuery.aspx?id=XML&div=1&dept=CSC
# and outputs it in CSV format.

import argparse
import csv
import sys
import xml.etree.ElementTree as ET


def update_terms(course, term):
    tbits = course.get('terms') or 0
    if 'Fall' in term:
        tbits |= 1
    if 'Spring' in term:
        tbits |= 2
    if 'Summer' in term:
        tbits |= 4
    course['terms'] = tbits


if __name__ == '__main__':
    usage = "%(prog)s < courses.xml > courses.csv"
    argparse.ArgumentParser(usage=usage).parse_args()

    tree = ET.parse(sys.stdin)
    root = tree.getroot()
    data = {}

    for c in root.findall('course'):
        cnum = c.find('cn').text
        # remove space and section number
        cnum = cnum[:cnum.find('-')].replace(' ', '')
        term = c.find('term').text

        if course := data.get(cnum):
            update_terms(course, term)
            continue
        course = {}
        course['title'] = c.find('title').text
        update_terms(course, term)
        course['credits'] = c.find('credits').text.strip()
        course['instructor'] = c.find('instructors').text
        course['description'] = c.find('description').text
        data[cnum] = course

    csvwriter = csv.writer(sys.stdout)
    csvwriter.writerow(['CourseNo', 'CourseTitle', 'TermsOffered', 'Credits',
                        'Instructor', 'Description'])
    for cnum, course in data.items():
        csvwriter.writerow([cnum] + list(course.values()))
