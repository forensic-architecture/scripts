# Tiling a mesh (DEM earth, v2)

# Geometry
1. Import the pristine object into Maya. We want only the mesh; so remove all of the other streaks, strata, and `Freeze Transformations` and delete history.
2. Decide on a size for the tile. This needs to divide evenly into the number of faces along one side. For example, if the mesh is 1000 by 665 faces, I might decide I want 10 faces along that side. 1000/20 = 50, so each tile will be 50 faces wide.
3. In the case that you need a square tile, cut the necessary number of faces from the other side in order to make it evenly divisible. E.g. 665 -> 650, as 650/50 = 13.
4. Highlight and delete the excess faces from the top of the mesh. You can work out which is the top of the mesh by showing the texture, and eyeballing it with the image. 
5. Save this file as **mesh-proportional.mb** (as a Maya binary). This mesh is now ready to be split into 260 50fx50f tiles (the mesh is 20x13 in tile terms). 
6. We now separate the mesh into its tiles. Ensuring that the variables at the top of this script are appropriate, run:
```
NO_OF_FACES = 50 # in each tile
MESH_NAME = "polySurface1"

from pymel.core import *

# manually discovered distance between across edges: but could maybe infer programmatically in standardized mesh.
def snd_hor(x):
	return (x-2)*3 + 4
def snd_ver(x):
	return (x-1)*2001 + 1001

FST_HOR = 2
SND_HOR = snd_hor(NO_OF_FACES)
FST_VER = 3
SND_VER = snd_ver(NO_OF_FACES)

polySelect(MESH_NAME, rpt=(FST_HOR,SND_HOR)) # across
polySelect(MESH_NAME, rpt=(FST_VER,SND_VER), add=True) # up
edgeIdxs = map(lambda e: e.index(), selected())
for idx in edgeIdxs:
    polySelect(MESH_NAME, el=idx, add=True)
```
* Edit Mesh -> Detach
* Deselect, select mesh in object mode
* Mesh -> Separate
* Edit -> Delete By Type -> History
* Remap the UV normals of all the objects. Select all the objects, and run the following MEL script
```
string $array[] = `ls -sl`;
for ($item in $array) {
  /* NB: this command just taken from UV Editor -> Create -> Automatic */
  polyAutoProjection -lm 0 -pb 0 -ibd 1 -cm 0 -l 2 -sc 1 -o 1 -p 6 -ps 0.2 -ws 0 $item;
};
```
* MEL script to export selected as individual obj files, renaming the export file appropriately, and making sure it exists.
```
global proc exportSelected()
{
string $mySelection[] = `ls -sl`;
for ($n=0 ; $n<size($mySelection) ; $n++)
	{
	select -r $mySelection[$n];
	file -force -options "(groups=1;ptgroups=1;materials=1;smoothing=1;normals=1" 
		 -typ "OBJexport" -pr 
		 -es ("/Users/forensicarchitecture/code/negev/split-texture/422x422_geo/tile_" + $n + ".obj");
	}
}
```
* Select all the objects, and run `exportSelected()` (in the MEL interpreter)
* Remove all the ‘.mtl’ by running `rm *.mtl` in the folder in bash.

# Image (texture)
Tiling images at the moment is a bit of a tradeoff-game. The tiles need to be exactly square (to the pixel), but the edges of the mesh for any given set of divisions often split pixels.
* It’s always necessary to skim off the number of pixels from the top that corresponds to the number of faces you deleted in step 4. 
* Then you basically have to decide where to drop pixels… I haven’t got an expressive algorithm for this yet, I just do it on an ad hoc basis. 