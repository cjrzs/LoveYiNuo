def gen(data):
    for item in data:
        if item % 2 == 0:
            yield item

data = [1, 2, 3, 4]
l1 = [item for item in data if item % 2 == 0]
print(l1)  #[2, 4]
gen1 = (item for item in data if item % 2 == 0)
print(gen1)  # <generator object gen...>
print(next(gen1))  # 2
print(next(gen1))  # 4
print(next(gen1))  # # StopIteration

gen2 = gen(data)
print(gen2)  # <generator object gen...>
print(next(gen2))  # 2
for item in gen2:
    print(item)  # 4
print(next(gen2))  # StopIteration



