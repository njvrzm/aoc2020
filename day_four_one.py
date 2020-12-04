#!env python
import sys

required = {'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}

def isValid(passport):
    return required.intersection(passport) == required

if __name__ == '__main__':
    valid = 0
    passport = dict()

    for line in open(sys.argv[1]):
        parts = dict(part.split(':') for part in line.split())
        if not parts:
            if isValid(passport):
                valid += 1
            passport = dict()
        else:
            passport.update(parts)
        

    if isValid(passport):
        valid += 1

    print(valid)