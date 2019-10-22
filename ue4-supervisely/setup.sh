#run `source setup.sh` in bash or zsh (no fish support) to setup this directory
#with the appropriate docker file.

# Get the current dir.
if [ -n "$BASH_VERSION" ]; then
	DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
elif [ -n "$ZSH_VERSION" ]; then
	DIR="$(dirname ${(%):-%N})"
else
	echo "Error: Unknown shell; cannot determine path to merantix/radler local repository"
fi
CORE_REPO_DIR="$(dirname $DIR)"
echo $CORE_REPO_DIR

BASE_VOLUMES="-v $CORE_REPO_DIR/ue4-supervisely:/safariland"
# TODO: make this next line portable
DATASET_VOLUMES="-v /home/fa-researcher/datasets:/datasets"
JUPYTER_PORT="8888"
JUPYTER_LAB_PORT="8889"
DOCKER_IMAGE_NAME="proj_safariland"

# if the image doesn't exist, build during setup
IMAGE_EXISTS=`docker images | grep "$DOCKER_IMAGE_NAME"`

if [[ -z "$IMAGE_EXISTS" ]]; then
	docker build --no-cache -t $DOCKER_IMAGE_NAME .
	echo "$DOCKER_IMAGE_NAME built."
else
		echo "$DOCKER_IMAGE_NAME already exists, no rebuilding. Remove image and run again if necessary."
fi

docker run -it $BASE_VOLUMES $DATASET_VOLUMES $ADDITIONAL_DRUN_PARAMS --rm $DOCKER_IMAGE_NAME
