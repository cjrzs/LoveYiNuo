from dis import dis
import sys

print(dis('a += 1'))


print(sys.getswitchinterval())  # 0.005
sys.setswitchinterval(0.006)
print(sys.getswitchinterval())  # 0.006


