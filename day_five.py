#!env python
def seat_id(code):
    return int(code.replace('F', '0')
               .replace('B','1')
               .replace('R','1')
               .replace('L','0'), 2)

if __name__ == '__main__':
    seat_ids = sorted(seat_id(code.strip()) for code in open('inputs/day_five.txt'))
    print("Part one: {}".format(seat_ids[-1]))
    last = seat_ids[0]
    for seat_id in seat_ids:
        if seat_id > last + 1:
            print("Part two: {}".format(last + 1))
            break
        last = seat_id
    


