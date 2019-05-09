# from subprocess import call
from os import listdir, makedirs, system

yrs = [1945, 1949, 1956, 1958, 1960, 1961, 1964, 1966, 1968, 1971, 1974, 1976, 1980, 1984, 1986, 1990, 1995, 1999, 2018]

for yr in yrs:
    dirname = './422x422_img_' + str(yr)
    new_dirname = './512x512_img_' + str(yr)

    paths = listdir(dirname)
    
    makedirs(new_dirname)

    for path in paths:
        nfilename = path[18:]
        ncmd = "convert {0} -resize 512x512 {1}".format(dirname+'/'+path, new_dirname+'/'+path)
        # print(ncmd)
        system(ncmd)
