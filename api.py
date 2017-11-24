import requests
import json
import time

API_KEY = '62fd3c20d21923e7caae5b0bab5771b5'
API_SEC = '5449d3ebbe1fe9dce4a941641c9d1e0e'
PRIM_EXCHANGE = 'BITF'

class ApiCoinigy:

    def __init__(self):
        self.headers = {
            'Content-Type': 'application/json',
            'X-API-KEY': API_KEY,
            'X-API-SECRET': API_SEC
        }       
        self.auth_id = self._get_auth_id()

    def _before_request(self):
        time.sleep(0.5)

    # Sinlge private outputs
    def _get_auth_id(self):
        """ There should always just be a single auth_id!"""
        self._before_request()
        r = requests.post('https://api.coinigy.com/api/v1/accounts', headers=self.headers).json()
        return r["data"][0]["auth_id"]

    # No outputs
    def update_balance(self):
        self._before_request()
        values = {
            "auth_id": self.auth_id
        }
        r = requests.post('https://api.coinigy.com/api/v1/refreshBalance', data=values, headers=self.headers).json()
        self._pprint(r)

    # Actions
    def add_order(self):
        self._before_request()
        values = {
            "auth_id": 1234,
            "exch_id": 62,
            "mkt_id": 125,
            "order_type_id": 2,
            "price_type_id": 3,
            "limit_price": 755,
            "order_quantity": 0.01
        }
        r = requests.post('https://api.coinigy.com/api/v1/addOrder', data=values, headers=self.headers).json()
        self._pprint(r)

    def cancel_order(self):
        self._before_request()
        values = {
            "internal_order_id":1234
        }
        r = requests.post('https://api.coinigy.com/api/v1/canelOrder', data=values, headers=self.headers).json()
        self._pprint(r)

    # Single outputs
    def get_userInfo(self):
        self._before_request()
        r = requests.post('https://api.coinigy.com/api/v1/userInfo', headers=self.headers).json()
        self._pprint(r)

    def get_orderTypes(self):
        self._before_request()
        r = requests.post('https://api.coinigy.com/api/v1/orderTypes', headers=self.headers).json()
        self._pprint(r)


    # Array-list outputs
    def list_exchanges(self):
        self._before_request()
        r = requests.post('https://api.coinigy.com/api/v1/exchanges', headers=self.headers).json()
        self._pprint(r)

    def list_orders(self):
        self._before_request()
        r = requests.post('https://api.coinigy.com/api/v1/orders', headers=self.headers).json()
        self._pprint(r)

    def list_balances(self):
        self._before_request()
        values = {
            "show_nils": 1,
            "auth_ids": self.auth_id
        }
        r = requests.post('https://api.coinigy.com/api/v1/balances', data=values, headers=self.headers).json()
        self._pprint(r)

    def list_markets(self):
        self._before_request()
        values = {
           "exchange_code": PRIM_EXCHANGE,
        }
        r = requests.post('https://api.coinigy.com/api/v1/markets', data=values, headers=self.headers).json()
        self._pprint(r)



    # Util functions
    def _pprint(self, r): 
        print(json.dumps(r, indent=2))

if __name__ == "__main__":
    apiObj = ApiCoinigy()
    print("User info")
    apiObj.get_userInfo()
    print("List balance")
    apiObj.list_balances()
    print("List order")
    apiObj.list_orders()
    print("get order")
    apiObj.get_orderTypes()
    print("update balance")
    apiObj.update_balance()
    print("add order")
    apiObj.add_order()
    print("list exchanges")
    apiObj.list_exchanges()
    print("markets")
    apiObj.list_markets()


