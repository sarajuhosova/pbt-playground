from hypothesis import given, strategies as st
from src.sort import insertion_sort, merge_sort

list_strategy = st.one_of(
    st.lists(st.integers()),
    st.lists(st.booleans()),
    st.lists(st.text())
)

@given(list_strategy)
def test_insertion_merge_sort(xs:list):
    assert insertion_sort(xs) == merge_sort(xs)