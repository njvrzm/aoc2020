#!/usr/local/bin/bash
#
# Example:
# $ ./day_three.sh "1 1,3 1,5 1,7 1,1 2,"
# 3584591857
# The trailing comma is annoying but :shrug:.
#

t=1
while read -d, right down; do
   c=$(awk -v right=$right -v down=$down '
       width==0 {
          width = length($0);
       }
       (NR - 1 + down) % down == 0 {
           if (substr($0, where + 1, 1)=="#") {trees++}
           where = (where + right) % width
       }
       END {
           print trees
       }' inputs/day_three.txt
   )

   t=$(( $c * $t))
done <<< $1
echo $t
