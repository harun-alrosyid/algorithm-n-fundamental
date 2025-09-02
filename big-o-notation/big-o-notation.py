
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
    if not data: raise ValueError("empty array")
    max = data[0]
    for val in data:
        if val > max:
            max = val
    return max

print(getMaxValue(data1))
# output : 9

# Big O(log n)

def firstPriceGte(prices, target):
    lo = 0
    hi = len(prices) - 1
    ans = -1
    while lo <= hi:
        mid = (lo + hi) // 2
        if prices[mid] >= target:
            ans = mid
            hi = mid - 1
        else:
            lo = mid + 1
    return prices[ans] if ans != -1 else None

data = [20000, 45000, 60000, 90000, 120000, 160000, 200000]
thresholdFreeDelivery = 150000

result = firstPriceGte(data, thresholdFreeDelivery)
print(result)
# output : 160000

# Big O(n^2)
tops = ["t-shirt", "shirt", "sweater"]
bottoms = ["jeans", "pants", "shorts"]

def outfitCombo(tops, bottoms):
    combos = []
    for t in tops:
        for b in bottoms:
            combos.append([t, b])
    return combos

print(outfitCombo(tops, bottoms))
# output :
# [['t-shirt', 'jeans'], ['t-shirt', 'pants'], ['t-shirt', 'shorts'], ['shirt', 'jeans'], ['shirt', 'pants'], ['shirt', 'shorts'], ['sweater', 'jeans'], ['sweater', 'pants'], ['sweater', 'shorts']]
