import sqlite3
import sys
import time
import pandas as pd
import numpy as np
from pandas import DataFrame
import matplotlib
import datetime

matplotlib.use('TkAgg')
import matplotlib.pyplot as plt

DISPLAY_RATE = 0.5

def establish_connection():
    conn = sqlite3.connect('/Users/davidal/documents/wolfofcrypto/src/database/sqlite_database.db')
    return conn, conn.cursor()


def get_all_records(cursor, currency):
    # must be one of these # turn this into an if or so
    assert (currency == "bitcoin" or currency == "ethereum" or currency == "litecoin")

    query = "SELECT * FROM " + currency
    res = cursor.execute(query)

    out = DataFrame(res.fetchall())
    out.columns = ["time", "market_cap", "price_btc", "price_usd", "vol_usd"]

    return out

def select_last_four_days(df):
    max_time = np.max(df.time)
    cropped_df = df[df.time > max_time - 86400] # Should be times 4

    return cropped_df

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
