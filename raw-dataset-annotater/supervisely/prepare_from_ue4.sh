#!/usr/bin/env bash
mkdir $2
mkdir $2/raw_ann
mkdir $2/img

cp $1/RTXCustomStencil* $2/raw_ann/
cp $1/RTXFinalImage* $2/img/

cd $2/raw_ann && rename 's/RTXCustomStencil\.//' *
cd $2/img && rename 's/RTXFinalImage\.//' *
