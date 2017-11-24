import json
import matplotlib.pyplot as plt
import numpy as np
import random
import sys
import datetime
import time
from src.environment.environment import *

class Agent():

    def __init__(self):
        self.NUM_EPOCHS = 300
        self.MAX_NUM_STEPS = 60 * 24 * 7 #one week in a 1-minute interval is maximum number of steps in an episode
        self.eps = 0.1
        self.gamma = 0.9
        self.cur_episode = 0
        self.reset_episode()
        self.reset_gamma()
        self.reset_eps()
        self.reset_episode()
         
    
    def random_action(self):
        return np.random.choice(self.env.action_space)
   
    def step_env(self, observation, reward, ):
        data, done = self.env.batch_load()
        self.env.reset_batch_counter()

        reward = 0
        a = np.mean(data[:, data_to["Price"]])
        b = np.mean(data[data.shape[0]//2:, data_to["Price"]])

        if a > b:
            action = 'short'
        elif a < b:
            action = 'long'
        else:
            action = 'hold'

        reward, done, actual_action = self.env.action(action, data[-1, data_to["Price"]])
#
#        #Q learning
#        Q_next = np.random.rand(1) * 500
#        maxQ_next = np.max(Q_next) #choose best from upcoming action
##        targetQ = all_Qs
##        targetQ[0, action] = reward + gamma * maxQ_next
#
#        #Update model to more optimal features
#
        return reward, done, actual_action

    def run_episode(self):
        total_reward = 0
        episode_done = False
        self.cur_episode += 1

        for step in range(self.MAX_NUM_STEPS):
            self.eps = self.eps/(self.cur_episode+1) + 0.06
            reward, done, action = self.step_env() 
            total_reward += reward

            if done:
                break

        self.env.close_all_positions()

        return total_reward, step
 
    def train(self):
        self.env.reset_positions()
        n = self.env.data.shape[0]

        reward_list = []
        steps_list = []
        for i in range(self.NUM_EPOCHS):

            start_time = datetime.datetime.now()
            reward, steps = self.run_episode()
            end_time = datetime.datetime.now()

            reward_list.append(reward)
            steps_list.append(steps)

            percentage_done = float(self.cur_episode)/self.NUM_EPOCHS
            total_time = end_time - start_time

#            print("Progress: {0:.3f}%%".format(percentage_done*100))
#            print("EST. time per episode: " + str(total_time))
#            print("Cur Episode: {0:d}".format(i))
#            if len(reward_list) == 6:
#                print("reward of last 10 episodes: {0:.3f} ".format(np.mean(reward_list[-6:])))

        return reward_list, steps_list


        
if __name__ == "__main__":
    dataLoader = DataLoader("/Users/davidal/Documents/blackmesa/src/api/data/ohlc_xethxxbt_", interval=5)
    train_data, cv_data, test_data = dataLoader.get_train_cv_test()
   
    para = {
            "start_train_date" : "2016-03-03 07:90:59", 
            "start_cv_date" : "2016-03-03 07:90:59",
            "start_test_date" : "2016-03-03 07:90:59",
            "percentage_train_cv_test" : True, # assumes a 0.6, 0.2, 0.2 split between cv, test, train sets
    }
    env = Environment(train_data,  para,  seq_length=3)


    agent = Agent(env)
    agent.train()

