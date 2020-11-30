# COV format folder name where new annotations will written
voc_folder_name = 'CanistersRealTrainVal'


# pattern to find the json files for the annotations
# for each json, there must be a png file with the same name
# but png extension, under imgs
json_path_pattern = ['more_37_40mm/*/*.json'] #'cansynth/2019*/*/*/*.json' #'not_can/*/*.json'
# pattern for the image files
img_patterns = ['more_37_40mm/*/*.png', 'more_37_40mm/*/*.jpg', 'more_37_40mm/*/*.jpeg']
# ['cansynth/2019*/*/*/*.png', 'cansynth/2019*/*/*/*.jpg', 'cansynth/2019*/*/*/*.jpeg']

# dataset name - different from voc_folder_name - used as prefix 
# for all files as well and to identify the dataset in other files
dataset = 'more_3740'
# prefix when creating the numbered XML files with VOC format annotations
prefix_im_name = dataset

# original canister classes and their mapping to dataset ids
classes_conversion = {'canister-general-bbox':0, 
                      'canister': 0, 
                      'triplechaser': 0,
                      'triple-bottom': 0,
                      'triple-top': 0,
                      'foambullet_bbox': 0,
                      'foambullet': 0,
                      'cylinder': 6, 
                      'can': 10, 
                      'bottle': 14, 
                      'bin': 18}

# mapping between the original canister classes to 
# the classes that will be used for training
class_mapping = {'canister-general-bbox': 'canister', 
                 'triplechaser': 'canister', 
                 'triple-bottom': 'canister', 
                 'triple-top': 'canister', 
                 'foambullet_bbox': 'canister', 
                 'foambullet': 'canister',
                 'canister': 'canister',
                 'cylinder': 'cylinder',
                 'can': 'can',
                 'bottle': 'bottle', 
                 'bin': 'bin',
                }
