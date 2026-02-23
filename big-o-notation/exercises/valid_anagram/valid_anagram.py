"""
Problem: Valid Anagram
Complexity:
- Time: O(n)
- Space: O(1) - alphabet size is constant.
"""

def is_anagram(s: str, t: str) -> bool:
    if len(s) != len(t):
        return False
    
    countS, countT = {}, {}
    
    for i in range(len(s)):
        countS[s[i]] = 1 + countS.get(s[i], 0)
        countT[t[i]] = 1 + countT.get(t[i], 0)
    
    return countS == countT

if __name__ == "__main__":
    s1, t1 = "anagram", "nagaram"
    s2, t2 = "rat", "car"
    
    print(f"s1: '{s1}', t1: '{t1}' -> Result: {is_anagram(s1, t1)}")
    print(f"s2: '{s2}', t2: '{t2}' -> Result: {is_anagram(s2, t2)}")
    
    if is_anagram(s1, t1) == True and is_anagram(s2, t2) == False:
        print("Test Passed! ✅")
    else:
        print("Test Failed! ❌")
