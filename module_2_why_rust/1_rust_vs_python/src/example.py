import helpers
import time

# Create a large list of numbers
numbers = helpers.large_list(10000000)

# Measure current time
start_time = time.perf_counter_ns()

# Sum all numbers
sum = 0
for number in numbers:
    sum = sum + number

# Measure elapsed time since start
end_time = time.perf_counter_ns()
elapsed = (end_time - start_time) / 1_000_000

print(f"Sum is {sum}, time it took is: {elapsed:.3f} ms")