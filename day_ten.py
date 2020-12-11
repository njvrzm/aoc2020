
from collections import Counter
from functools import reduce
import itertools

def product(seq):
    return reduce(int.__mul__, seq)

def part_one(gaps):
    return product(Counter(gaps).values())

def part_two(gaps):
    groups = [list(group) for _, group in itertools.groupby(gaps)]
    ones_lengths = [len(group) for group in groups if group[0] == 1]
    return product([tribonacci(l) for l in ones_lengths])

def tribonacci(n):
    a, b, c = 0, 0, 1
    for _ in range(n):
        a, b, c = b, c, a+b+c
    return c


if __name__ == '__main__':
    numbers = sorted([int(l) for l in open('inputs/day_ten.txt')])
    gaps = [1] + [numbers[i+1] - numbers[i] for i in range(len(numbers)-1)] + [3]
    print(part_one(gaps))
    print(part_two(gaps))

