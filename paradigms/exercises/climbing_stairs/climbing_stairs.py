"""
Problem: Climbing Stairs
"""

def climb_stairs(n: int) -> int:
    if n <= 2:
        return n
    
    # Ways to reach step 1 and step 2
    two_steps_before = 1
    one_step_before = 2
    
    for _ in range(3, n + 1):
        current = one_step_before + two_steps_before
        two_steps_before = one_step_before
        one_step_before = current
        
    return one_step_before

if __name__ == "__main__":
    test_n = 5
    result = climb_stairs(test_n)
    print(f"Steps: {test_n}, Distinct ways: {result}")
    
    if climb_stairs(2) == 2 and climb_stairs(3) == 3 and result == 8:
        print("Test Passed! ✅")
    else:
        print("Test Failed! ❌")
