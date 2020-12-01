import glob, shutil
import config as cfg

def remove_last_occurence(mystr, removal, replacement):
    reverse_removal = removal[::-1]
    reverse_replacement = replacement[::-1]
    return mystr[::-1].replace(reverse_removal, reverse_replacement, 1)[::-1]


def rename_files():
    for pattern in cfg.img_patterns:
        for file_path in glob.glob(pattern):
            print(file_path)
            new_path = file_path.replace('.', '_', file_path.count('.')-1)
            new_path = new_path.replace('_jpg', '')
            new_path = new_path.replace('_jpeg', '')
            new_path = new_path.replace('_png', '')
            new_path = new_path.replace('.jpeg', '.png')
            new_path = new_path.replace('.jpg', '.png')
            new_path = new_path.replace(' ', '_')
            shutil.move(file_path, new_path)


    for pattern in cfg.json_path_pattern:
        for file_path in glob.glob(pattern):
            print(file_path)
            new_path = remove_last_occurence(file_path, '.jpeg', '')
            new_path = remove_last_occurence(new_path, '.jpg', '')
            new_path = remove_last_occurence(new_path, '.png', '')
            new_path = file_path.replace('.', '_', file_path.count('.')-1)
            new_path = new_path.replace('_jpg', '')
            new_path = new_path.replace('_jpeg', '')
            new_path = new_path.replace('_png', '')
            new_path = new_path.replace(' ', '_')
            shutil.move(file_path, new_path)
        
if __name__ == "__main__":
    rename_files()