from typing import Set, Tuple

# If this isn't sloth, idk what is

results: Set[Tuple[int, int, int]] = set()

for first in range(0, 37):
    for second in range(first, 37):
        for third in range(second, 37):
            prod = first * second * third
            if prod == 36:
                arr = [first, second, third]
                arr.sort()
                results.add((arr[0], arr[1], arr[2]))
            if prod > 36:
                break

print(results)
