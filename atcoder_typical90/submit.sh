#!/bin/bash

submit() {
  echo $1
  for i in {0..$1}; do
    echo $2
  done
}

DIR="./${1}/*"
IN="in"
OUT="out"
SOLVERNAME="solve.py"

SOLVER="${1}/$SOLVERNAME"
if [ ! -e $SOLVER ]; then
	echo "solver is not found."
	exit 
fi

IN_FILES=()
OUT_FILES=()

for f in $DIR; do
  EXT=${f##*.}
  if [ $EXT = $IN ]; then
	  IN_FILES+=(${f})
  elif [ $EXT = $OUT ]; then
    OUT_FILES+=(${f})
  fi
done

if [ ${#IN_FILES[@]} -ne ${#OUT_FILES[@]} ]; then
  echo "number of input and output files is not matched."
  exit 0
fi

LEN=`expr ${#IN_FILES[@]} - 1`
for i in `seq 0 $LEN`; do
  I=`expr ${i} + 1`
  python $SOLVER < ${IN_FILES[$i]} > tmp;
  sed 's/\s*$//' tmp > out;
  RET=`diff ${OUT_FILES[$i]} out`;
  if [ $? -eq 0 ]; then
    printf "%02d AC\n" "${I}"
  else
    printf "%02d WA\n" "${I}"
  fi
done 

rm out tmp
exit 0
