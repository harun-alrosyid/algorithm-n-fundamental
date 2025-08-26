
# Big O(1)
data = [1, 2, 3, 4, 5]

def getValueItem(n):
    return data[n]

print(getValueItem(2))
# output : 3
print(getValueItem(4))
# output : 5

data1 = [1,2,6,4,5,9,0,3]

# Big O(n)
def getMaxValue(data):
    max = data[0]
    for val in data:
        if val > max:
            max = val
    return max

print(getMaxValue(data1))
# output : 9
