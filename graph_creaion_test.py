import time
import sys
import math

start_time = time.time()

try:
    n = int(sys.argv[1])
except IndexError:
    n = int(input("n = "))
    
m = sum([i for i in range(n-1)])

get_n_connections = lambda n: int(math.log2(n)) + 2
n_connections = (int(get_n_connections(n)/2)+1)*(n-2)

print(n,m, n_connections)

def new_perm(last_perm):
    global n_connections
    new_perm_list = []
    for perm in last_perm:
        new_perm_list.append([*perm, 0])
        if not perm.count(1) >= n_connections:
            new_perm_list.append([*perm, 1])
    return new_perm_list

last_perm = [[]]
for i in range(m):
    print(i)
    last_perm = new_perm(last_perm)

"""
print(len(last_perm))
for perm in last_perm:
    print(perm)"""
  
print(f"executed in {time.time()-start_time}s")
  
print(len(last_perm))
print(last_perm[-1])

