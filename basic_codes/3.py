############# Random  #####################
import random

outcome = ["rock", "paper", "scissors"]

for i in range(1):
    print("Random value", random.random())
    print("Random uniform", random.uniform(3, 7))
    print("Random bell curve", random.normalvariate(5, 1))
    print("Random integer", random.randint(5, 15))
    print("Random choice", random.choice(outcome))
    print()


############# CSV file  #####################

import csv
from datetime import datetime

path = "/home/sudhanshu/Desktop/Python/work/gsp.csv"
file = open(path, newline="")
reader = csv.reader(file)
header = next(reader)
data = []
# data =[row for row in reader]
# print(header)
# print(data[0])
for row in reader:
    date = datetime.strptime(row[0], "%m/%d/%Y")
    open_price = float(row[1])
    high = float(row[2])
    low = float(row[3])
    close = float(row[4])
    volume = int(row[5])
    adj_close = float(row[6])

    data.append([date, open_price, high, low, close, volume, adj_close])
# print(header)
# print(data[0])

return_path = "/home/sudhanshu/Desktop/Python/work/gsp_return.csv"
file = open(return_path, "w")
writer = csv.writer(file)
writer.writerow(["Date", "Return"])

for i in range(len(data) - 1):
    today_row = data[i]
    today_date = today_row[0]
    today_price = today_row[-1]
    yes_row = data[i + 1]
    yes_price = yes_row[-1]

    daily_return = (today_price - yes_price) / yes_price
    formatted_date = today_date.strftime("%m/%d/%Y")
    writer.writerow([formatted_date, daily_return])


################### Random walk ###########################

outcome_walk = [0, 1, -1]


def random_walk_fn(n):
    x, y = 0, 0
    for i in range(n):
        dx = random.choice(outcome_walk)
        dy = random.choice(outcome_walk)
        x += dx
        y += dy
    return (x, y)


# for i in range(20):
#     walk = random_walk_fn(10)
#     print(walk,"Distance from home = ",abs(walk[0])+abs(walk[1]))

no_walk = 10000
for walk_length in range(1, 31):
    no_trans = 0
    for i in range(no_walk):
        (x, y) = random_walk_fn(walk_length)
        distance = abs(x) + abs(y)
        if distance <= 4:
            no_trans += 1
    no_trans_per = float(no_trans) / no_walk
    print(
        "Walk size = ",
        walk_length,
        " Distance = ",
        distance,
        " % of no transport = ",
        100 * no_trans_per,
    )
