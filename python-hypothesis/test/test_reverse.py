from hypothesis import given, strategies as st
from src.reverse import my_reverse

singleton_strategy = st.one_of(
    st.lists(st.integers(), min_size=1, max_size=1),
    st.lists(st.booleans(), min_size=1, max_size=1),
    st.lists(st.text(), min_size=1, max_size=1)
)

list_strategy = st.one_of(
    st.lists(st.integers()),
    st.lists(st.booleans()),
    st.lists(st.text())
)

@given(st.lists(st.integers(), min_size=1, max_size=1))
def test_revese_one_element(xs:list):
    assert my_reverse(xs) == xs
    
@given(list_strategy)
def test_reverse_invariant(xs:list):
    assert my_reverse(my_reverse(xs)) == xs

@given(list_strategy)
def test_reverse_length_invariant(xs:list):
    assert len(my_reverse(xs)) == len(xs)