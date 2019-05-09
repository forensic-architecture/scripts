FILE_EXT=jpg

FOLDER_NAME=$1

crop_and_tile () {
  # skim 127 pixels from the top 
  convert $1 -crop 8440x5499+0+133 $1-int1.$FILE_EXT
  mkdir 422x423_img_$2
  convert $1-int1.$FILE_EXT +repage -crop 422x423 422x423_img_$2/img_%d.$FILE_EXT
  #cleanup
  rm $1-int1.$FILE_EXT
  # rm $1-int2.$FILE_EXT
}

for filename in $FOLDER_NAME*
do
  FILE=`echo $filename| cut -d'/' -f 2`
  YEAR=`echo $FILE| cut -d'_' -f 1`
  crop_and_tile $filename $YEAR
done
