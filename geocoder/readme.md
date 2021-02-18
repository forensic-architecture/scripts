# Geo Coder

Takes a CSV, returns a CSV with Longitude and Latitude. No more Google Map cut and paste.

Uses: OpenStreetMap

## Install it

Setup:

Python and Pip set up

Install Python and pip3 then use Pip to set up your Python local environment:

```
pip install virtualenv
virtualenv geocoder-env
source geocoder-env/bin/activate
```

Install dependencies:

`pip install -r requirements.txt`

## Run it

`python geoCoder.py search-locations.csv` (or the file of your choice)

## In

A CSV with two columns:

* state
* city

Replace:

`California,Los Angeles`

With your locations.

## Out

`geocoded-output.csv`

A CSV output with the following fields:

* city - the city you passed in
* state	- the state you passed in
* latitude - the latitude found	
* longitude	- the longitude found
* display_name - the location found so you can check against your input

e.g. 

`Los Angeles,California,34.0536909,-118.242766,"Los Angeles, Los Angeles County, California, United States"`
