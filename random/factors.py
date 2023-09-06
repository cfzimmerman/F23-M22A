from typing import Set, Tuple

# If this isn't sloth, idk what is

TARGET = 36

results: Set[Tuple[int, int, int]] = set()

for first in range(0, TARGET + 1):
    for second in range(first, TARGET + 1):
        for third in range(second, TARGET + 1):
            prod = first * second * third
            if prod == 36:
                arr = [first, second, third]
                arr.sort()
                results.add((arr[0], arr[1], arr[2]))
            if prod > 36:
                break

print(results)
