import csv as csv
import requests
import sys
import json
import socket
from socket import timeout


# add links to be ignored
ignoreList = []

timeout = 5
socket.setdefaulttimeout(timeout)

headers = {
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3729.169 YaBrowser/19.6.1.153 Yowser/2.5 Safari/537.36'}

def main():
    fileLocation = ''
    for arg in sys.argv[1:]:
        fileLocation = arg
    urlsToVerify = getJSONFields(fileLocation)
    results = []
    for urlItem in urlsToVerify:
        results.append(getUrl(urlItem))

    csvOutputFile = 'results.csv'
    with open(csvOutputFile, 'w', newline='') as csvOutputFile:
        fieldnames = ['error', 'type', 'url', 'id']
        writer = csv.DictWriter(csvOutputFile, fieldnames)
        writer.writeheader()
        writer.writerows(results)

def getUrl(item):
    errors = []
    id = item['id']
    type = item['type']
    url = item['url']
    if (url == ''):
        return {'error': 'success', 'type': type, 'url': 'empty-url', 'id': id}
    if (url in ignoreList):
        return {'error': 'success', 'type': type, 'url': url, 'id': 'ignore'}

    try:
        response = requests.get(item['url'], headers=headers, timeout=5)
        if (not response.ok):
            print(response.ok)
            print(id)
            print(url)
            return {'error': '404', 'type': type, 'url': url, 'id': id}
    except (requests.exceptions.Timeout):
        print('[timeout]', id)
        return {'error': 'timeout', 'type': type, 'url': url, 'id': id}

    except (requests.exceptions.ConnectionError):
        print('[connection]', id)
        return {'error': 'connection', 'type': type, 'url': url, 'id': id}

    except requests.exceptions.RequestException as e:
        print(e, id)
        print(e, url)

        return {'error': e, 'type': type, 'url': url, 'id': id}

    return {'error': 'success', 'type': type }

def getJSONFields(file):
    results = []
    with open(file, newline='') as file:
        data = json.load(file)

    for object in data:
        jsonObject = data[object]
        id = jsonObject['id']
        
        url = jsonObject['url'].strip()
        results.append({'id': id, 'url': url, 'type': 'url'})

        thumbnail = jsonObject['thumbnail'].strip()
        results.append({'id': id, 'url': thumbnail, 'type': 'thumbnail'})

        paths = jsonObject['paths'] 
        for path in paths:
            pathUrl = path.strip()
            results.append({'id': id, 'url': pathUrl, 'type': 'path'})
    return results
 

if __name__ == "__main__":
    main()
