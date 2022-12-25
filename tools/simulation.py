# This file is intended to be run within VS Code in interactive mode

# %%
import math
import random

import numpy as np
from matplotlib.pyplot import *


class DynamicSmootherEco():
    def __init__(self, basefreq, samplerate, sensitivity):
        wc = basefreq / samplerate
        gc = math.tan(wc * math.pi)
        g0 = 2 * gc / (1 + gc)

        self.low1 = 0
        self.low2 = 0
        self.g0 = g0
        self.sense = sensitivity * 4

    def clear(self):
        self.low1 = 0
        self.low2 = 0

    def tick(self, input):
        low1z = self.low1
        low2z = self.low2
        bandz = low1z - low2z
        g = min(self.g0 + self.sense * abs(bandz), 1)
        self.low1 = low1z + g * (input - low1z)
        self.low2 = low2z + g * (self.low1 - low2z)

        return self.low2


smoother1 = DynamicSmootherEco(0.1, 100, 0.01)

NUM_SAMPLES = 100
random.seed(0)
x = np.arange(NUM_SAMPLES)
input1 = 1000
input2 = 500
y = [smoother1.tick(input1 + random.randint(-50, 50))
     for i in range(NUM_SAMPLES // 2)]
y += [smoother1.tick(input2 + random.randint(-50, 50))
      for i in range(NUM_SAMPLES // 2)]

plot(x, y)

# %%
