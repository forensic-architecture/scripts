# COV format folder name where new annotations will written
voc_folder_name = 'Canisters_2020'
original_dataset_name = 'not_can'

# dataset name - different from voc_folder_name - used as prefix 
# for all files as well and to identify the dataset in other files
dataset = 'not_can_val'
# prefix when creating the numbered XML files with VOC format annotations
prefix_im_name = dataset

# pattern to find the json files for the annotations
# for each json, there must be a png file with the same name
# but png extension, under imgs
json_path_pattern = [f'{original_dataset_name}/*/*.json'] #'cansynth/2019*/*/*/*.json' #'not_can/*/*.json'
# json_path_pattern = ['cansynth/2019*/*/*/*.json']
# pattern for the image files
img_patterns = [f'{original_dataset_name}/*/*.png', f'{original_dataset_name}/*/*.jpg', f'{original_dataset_name}/*/*.jpeg']
# img_patterns = ['cansynth/2019*/*/*/*.png', 'cansynth/2019*/*/*/*.jpg', 'cansynth/2019*/*/*/*.jpeg']



# original canister classes and their mapping to dataset ids
classes_conversion = {'canister-general-bbox':0, 
                      'triplecanister_top_bbox':0,
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
                 'triplecanister_top_bbox': 'canister',
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

# original canister classes and their mapping to dataset ids
classes_conversion_1shot = {'canister-general-bbox':2, 
                      'triplecanister_top_bbox':2,      
                      'canister': 2, 
                      'triplechaser': 2,
                      'triple-bottom': 2,
                      'triple-top': 2,
                      'foambullet_bbox': 2,
                      'foambullet': 2,
                      'cylinder': 6, 
                      'can': 10, 
                      'bottle': 14, 
                      'bin': 18}