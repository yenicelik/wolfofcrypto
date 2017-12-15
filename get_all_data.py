# https://graphs.coinmarketcap.com/currencies/bitcoin/1367131641000/1367218041000/
import requests
import datetime
import sys
import sqlite3
import time
conn = sqlite3.connect('coinmarketcap5min.db')
c = conn.cursor()


#c.execute('''DROP TABLE ethereum;''')
#sys.exit()

#ETHEREUM ethereum base=1438958970000 https://graphs.coinmarketcap.com/currencies/ethereum/1438958970000/1485204592442/
#BITCOIN bitcoin base=1367131641000

c.execute('''CREATE TABLE ethereum
             (time INTEGER PRIMARY KEY, market_cap REAL, price_btc REAL, price_usd REAL, vol_usd REAL)''')
conn.commit()

#sys.exit(0)

base = 1438958970000
off = abs(1438958970000 - 1485204592442)

while True:
    time.sleep(1)
    r = requests.get("https://graphs.coinmarketcap.com/currencies/ethereum/" + str(base) + "/" + str(base+off) +
                     "/").json()

    market_cap = r["market_cap_by_available_supply"]
    price_btc = r["price_btc"]
    price_usd = r["price_usd"]
    vol_usd = r["volume_usd"]

    for i in range(len(market_cap)):
        assert(market_cap[i][0] == price_usd[i][0] and vol_usd[i][0] == price_btc[i][0] and price_usd[i][0] == price_btc[i][0])
        print(int(market_cap[i][0]/1000), market_cap[i][1], price_btc[i][1], price_usd[i][1], vol_usd[i][1])

        unixtime = int(market_cap[i][0]/1000)
        val1 = market_cap[i][1]
        val2 = price_usd[i][1]
        val3 = price_btc[i][1]
        val4 = vol_usd[i][1]
        c.execute("INSERT INTO ethereum VALUES (?, ?, ?, ?, ?);", (unixtime, val1, val2, val3, val4))

    print("Last value: ", market_cap[i][0])
    conn.commit()

    base += off


conn.commit()
conn.close()
