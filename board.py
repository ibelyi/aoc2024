from datetime import datetime
from json import JSONDecodeError, load as jsonLoad
import sys

if len(sys.argv) > 1:
    filename = sys.argv[1]
else:
    filename = 'board.json'

try:
    with open(filename) as json_file:
        data = jsonLoad(json_file)
        for value in data['members'].values(): 
            print(value['name'])
            result = value['completion_day_level']
            for day in range(1,26):
                entry = result.get(str(day))
                if entry:
                    time = [datetime.fromtimestamp(x.get('get_star_ts')).strftime('%Y-%m-%d %H:%M:%S')
                            for x in [entry.get(k) for k in ['1','2']] if x]
                    print(f'Day {day}: {", ".join(time)}')
except FileNotFoundError:
    print(f'File "{filename}" is not found')
except JSONDecodeError:
    print(f'File "{filename}" is not a JSON file')
except TypeError as e:
    print(f'File "{filename}" is not a Private Leaderboard JSON file: {e}')
except KeyError as e:
    print(f'File "{filename}" is not a Private Leaderboard JSON file: missing key {e}')

