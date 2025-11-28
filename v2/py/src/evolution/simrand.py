import random

RANDOM_UNSEEDED = random.Random()
SEED = RANDOM_UNSEEDED.randrange(1000)
RANDOM = random.Random(SEED)
