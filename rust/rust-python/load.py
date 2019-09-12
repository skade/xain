import numpy as np

xs = np.load('after.npy')
assert np.array_equal(xs, np.arange(1, 11, dtype = np.double))
