FILE_EXT=jpg

FOLDER_NAME=$1

crop_and_tile () {
  # skim 55 pixels from the top 
  convert $1 -crop 8448x5577+0+55 $1-int1.$FILE_EXT
  # add 2 white pixels to the right
  convert $1-int1.$FILE_EXT -background white -gravity northeast -splice 2x0 $1-int2.$FILE_EXT
  mkdir 169x169_img_$2
  # tile in 169x169 panels
  convert $1-int2.$FILE_EXT +repage -crop 169x169 169x169_img_$2/img_%d.$FILE_EXT
  #cleanup
  rm $1-int1.$FILE_EXT
  rm $1-int2.$FILE_EXT
}

for filename in $FOLDER_NAME*
do
  FILE=`echo $filename| cut -d'/' -f 2`
  YEAR=`echo $FILE| cut -d'_' -f 1`
  crop_and_tile $filename $YEAR
done
