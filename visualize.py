import sqlite3
import sys
import time
import pandas as pd
import numpy as np
from pandas import DataFrame
import datetime
import requests
import time

from time import mktime

import matplotlib
matplotlib.use('TkAgg')
import matplotlib.pyplot as plt

# CONSTANTS
DISPLAY_RATE = 0.5
SLIDING_DAYS = 100
COLUMN_NAMES = ["time", "market_cap", "price_btc", "price_usd", "vol_usd"]

#TODO: implement such that each pair get's their own line (and then calculate the portfolio value

def get_earliest_date():

    #TODO

    pass

def get_latest_date():

    #TODO

    pass

def check_if_server_is_working():
    r = requests.get('http://localhost:8000/')
    print(r)

def get_last_days_data(days):
    # Check if input is correct
    assert(isinstance(days, type(5)))
    assert(days > 0)

    data = {
        'start_unixtime': round(time.time()) - (24 * 60 * 60) * days, #round(time.time()),
        'end_unixtime': round(time.time()), # round(time.time()) - (24 * 60 * 60) * days,
        'currency': None
    }

    r = requests.post("http://localhost:8000/historical/get-data-from-to/", json=data)
    result = r.json()['result']
    for i in result:
        print(i)

def get_between(start_unixtime, end_unixtime):
    # Check if input is correct
    assert(isinstance(start_unixtime, type(5)))
    assert(isinstance(end_unixtime, type(5)))
    assert(start_unixtime > 0)
    assert(end_unixtime > 0)

    print("Start time: ", start_unixtime)
    print("End time: ", end_unixtime)
    print(datetime.datetime.fromtimestamp(start_unixtime).strftime('%Y-%m-%d %H:%M:%S'))
    print(datetime.datetime.fromtimestamp(end_unixtime).strftime('%Y-%m-%d %H:%M:%S'))


    data = {
        'start_unixtime': start_unixtime,
        'end_unixtime': end_unixtime,
        'currency': None
    }

    r = requests.post("http://localhost:8000/historical/get-data-from-to/", json=data)

    # Check if r['error'] is empty
    if r.json()['error'] != "":
        print("Error happened")
        print(r.json()['error'])
        sys.exit(0)

    result = r.json()['result']
    out = DataFrame(result)
    return out


def plot_single_frame(ax1, data):
    # TODO: Check if input is non-empty
    dates=[datetime.datetime.fromtimestamp(ts) for ts in data.time]
    print("Plotting")
    plt.cla()
    ax1.plot(dates, data.price_usd, 'r')
    plt.pause(DISPLAY_RATE)

def show_live():
    # Create plot
    fig = plt.figure()
    plt.ion()
    ax1 = fig.add_subplot(111, ylabel="Price of BTC in USD")

    # Setting up parameters
    start_time_date = datetime.date(2014, 1, 7) #1st August 2013
    start_unixtime = int(mktime(start_time_date.timetuple()))
    period = 24 * 60 * 60 * SLIDING_DAYS

    while True:
        print("Start unixtime window: ", start_unixtime)
        display_data = get_between(start_unixtime, start_unixtime + period)
        plot_single_frame(ax1, display_data)
        start_unixtime += period // 10


if __name__ == "__main__":

    try:
        check_if_server_is_working()
    except Exception as inst:
        print("Server is not connecting....")
        sys.exit(99)

    show_live()

    # #  #   short_times = [get_time(y) for y in self.all_short_positions]
    # #
    # #  #   in_shorts = lambda x: 1 if x in short_times else 0
    # #
    # #  #   #print(self.all_short_positions[0])
    # #
    # #  #   #print([in_shorts(x) for x in self.data[:, conf.data_to["Date"]]])
    # #
    # #  #   print(len(self.all_short_positions))
    # #  #   print(self.data.shape[0])
    # #
    # #
    # #  #   ax1.plot(
    # #  #   [get_time(x) for x in self.all_short_positions],
    # #  #   [get_price(x) for x in self.all_short_positions],
    # #  #   'v', markersize=9, color='r'
    # #  #   )
    # #
    # #  #   ax1.plot(
    # #  #   [get_time(x) for x in self.all_long_positions],
    # #  #   [get_price(x) for x in self.all_long_positions],
    # #  #   '^', markersize=9, color='b')
    # #
    # #  #   plt.savefig('/Users/davidal/Documents/blackmesa/sample.png')
    # #
    # #  #   plt.show()
    # #
    # #
    # # if __name__ == "__main__":
    # #     display_actions()
