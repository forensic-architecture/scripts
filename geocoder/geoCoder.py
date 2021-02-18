import csv as csv
import sys
import geopy
from geopy.geocoders import Nominatim
from geopy.extra.rate_limiter import RateLimiter


def main():
    csvFile = ''
    for arg in sys.argv[1:]:
        csvFile = arg
    locations = getLocations(csvFile)
    locationsResult = []

    for location in locations:
        locator = Nominatim(user_agent='myGeocoder')
        locationQuery = {'city': location['city'], 'state': location['state']}
        locationGeocoded = locator.geocode(locationQuery)

        geocode = RateLimiter(locator.geocode, min_delay_seconds=1)
        location_city = location['city']
        location_state = location['state']
        if locationGeocoded:
            raw = locationGeocoded.raw
            print(raw)
            stripped = {'city': location_city,
                        'state': location_state,
                        'latitude': raw['lat'],
                        'longitude': raw['lon'],
                        'display_name': raw['display_name']
                        }
            locationsResult.append(stripped)
        else:
            locationsResult.append({'city': location_city,
                                    'state': location_state,
                                    'latitude': 'NOT_FOUND',
                                    'longitude': 'NOT_FOUND',
                                    'display_name': 'NOT_FOUND'}
                                   )
    csvOutputFile = 'geocoded-output.csv'
    with open(csvOutputFile, 'w', newline='') as csvOutputFile:
        fieldnames = ['city', 'state', 'latitude', 'longitude', 'display_name']
        writer = csv.DictWriter(csvOutputFile, fieldnames)
        writer.writeheader()
        writer.writerows(locationsResult)


def getLocations(file):
    locations = []
    with open(file, newline='') as file:
        reader = csv.DictReader(file)
        for row in reader:
            city = row['city']
            state = row['state']
            locations.append({'city': city, 'state': state})
    return locations


if __name__ == "__main__":
    main()
