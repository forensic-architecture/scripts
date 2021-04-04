# Link Checker

This is a script that batch verifies a list of source urls to see if they are broken. It works with the JSON format for timemap sources so you should be able to copy the output from the datasheet-server sources tab.

## Install

Clone this repo from GitHub.

## Python and Pip set up

Install Python and pip3 then use Pip to set up your Python local environment:

```
pip install virtualenv
virtualenv scripts-env
source scripts-env/bin/activate
```

Install dependencies:

`pip install -r requirements.txt`

## Run

Copy the JSON format sources from datasheet server into the sources.json in this directory.

run:

`python linkChecker.py sources.json`

Once it completes it creates a file called `results.csv` that you can open and check all the urls that have been verified. The results include both success and errors where: 

* `success` - the url is correct. If the url path, url or thumbnail are empty you get a success.

* `404` - this is most likely because the url is no longer there (the content has been moved) or the url is wrong and you need to check that it is correct.

* other errors - these can occur because a server is no longer there, the request times out, or a multitude of other reasons.  

## ignoring some urls

Some urls mau not be accessible in your geographic region but are accessible outside. If you have urls like this add them to the `ignoreList` array in `linkChecker.py`. Any ignored urls are included in the results of the `results.csv`
