from hypothesis import given, strategies as st
from src.min_max import my_min, my_max

@given(st.integers())
def test_min_invariant(a:int):
    assert my_min(a, a) == a
    
@given(st.integers(), st.integers())
def test_max_associative(a:int, b:int):
    assert my_max(a, b) == my_max(b, a)
    
@given(st.integers(), st.integers())
def test_min_max_opposite(a:int, b:int):
    low = my_min(a, b)
    high = my_max(a, b)
    if low == a: assert high == b
    if low == b: assert high == a
    