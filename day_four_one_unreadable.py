import sys
print(sum(1 for chunk in open(sys.argv[1]).read().split('\n\n')
    if len({'byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid'}.intersection(
        dict(part.split(':') for part in chunk.split()))) == 7))

