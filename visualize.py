import sqlite3
import sys
import time
import pandas as pd
import numpy as np
from pandas import DataFrame
import datetime
import requests
import time

import matplotlib
matplotlib.use('TkAgg')
import matplotlib.pyplot as plt

# CONSTANTS
DISPLAY_RATE = 0.5
COLUMN_NAMES = ["time", "market_cap", "price_btc", "price_usd", "vol_usd"]

def get_last_24h_data():
    days = 4
    data = {
        'start_unixtime': round(time.time()),
        'end_unixtime': round(time.time()) * (24 * 60 * 60) * days,
        'currency': None
    }
    r = requests.post("http://bugs.python.org", data=data)
    print(r.json())

def plot_single_frame(ax1, data):
    dates=[datetime.datetime.fromtimestamp(ts) for ts in data.time]
    ax1.plot(dates, data.price_usd, 'r')
    plt.pause(DISPLAY_RATE)


def show_live():
    # Establish database connection
    conn, cursor = establish_connection()

    # Create plot
    fig = plt.figure()
    plt.ion()

    ax1 = fig.add_subplot(111, ylabel="Price of BTC in USD")
    i = 0

    while True:
        all_records = get_all_records(cursor, "bitcoin")
        last_4_days = select_last_four_days(all_records)
        plot_single_frame(ax1, last_4_days)


if __name__ == "__main__":
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
