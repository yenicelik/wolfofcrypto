import sys
import numpy as np
import pandas as pd
import time
import src.config as config

class Environment:

    def __init__(self):
        self.basecurrency = "BTC"
        self.tradecurrency = "ETH"
        
  	self.batch_counter = 0
	self.batch_done = False
 	self.time_counter = 0

   	self.positions = []
	self.positions_long = 0
	self.all_long_positions = []
	self.all_short_positions = []





    def reset_pos_counter(self):
        self.pos_counter = 0

    def reset_positions(self):
        self.positions = []
        self.positions_long = 0 #1 means long, 0 means none, -1 means short
        self.all_long_positions = []
        self.all_short_positions = []

    def reset_portfolio(self):
        self.portfolio = {}

    def reset_PnL(self):
        self.PnL = 0.

    def reset_env(self):
        self.reset_btc()
        self.reset_eth()
        self.reset_batch_counter()
        self.reset_positions()
        self.reset_portfolio()
        self.reset_PnL()
        self.reset_maximizing_currency()
        self.reset_max_position_volume()

    def reset_max_position_volume(self):
        self.max_position_volume = 15 #This should actually be tied to the volume in the base currency the user has
