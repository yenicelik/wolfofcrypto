import pandas as pd
import numpy as np
import matplotlib
matplotlib.use('TkAgg')
import matplotlib.pyplot as plt

def import_csv():
    dataframe = pd.read_csv("./data/bitcoin.csv")
    print(dataframe.head(5))
    return dataframe


""" Display functions """
def display_actions():
    # Import data
    data = import_csv()

    # Create plot
    fig = plt.figure()
    plt.ion()
    ax1 = fig.add_subplot(111, ylabel="Price of BTC in USD")

    i = 0
    step = len(data.time)//10

    for k in range(10):
        ax1.plot(data.time[i:i+step], data.price_btc[i:i+step])
        plt.pause(2)
        i += step


    plt.show()


 #   # Create plot
 #   fig = plt.figure()
 #   ax1 = fig.add_subplot(111, ylabel="Price of BTC in USD")
 #   ax1.plot(self.data[:, 0], self.data[:, 2])

 #   get_time = lambda x: x[2]
 #   get_price = lambda x: x[1]

 #   short_times = [get_time(y) for y in self.all_short_positions]

 #   in_shorts = lambda x: 1 if x in short_times else 0

 #   #print(self.all_short_positions[0])

 #   #print([in_shorts(x) for x in self.data[:, conf.data_to["Date"]]])

 #   print(len(self.all_short_positions))
 #   print(self.data.shape[0])


 #   ax1.plot(
 #   [get_time(x) for x in self.all_short_positions],
 #   [get_price(x) for x in self.all_short_positions],
 #   'v', markersize=9, color='r'
 #   )

 #   ax1.plot(
 #   [get_time(x) for x in self.all_long_positions],
 #   [get_price(x) for x in self.all_long_positions],
 #   '^', markersize=9, color='b')

 #   plt.savefig('/Users/davidal/Documents/blackmesa/sample.png')

 #   plt.show()


if __name__ == "__main__":
    display_actions()