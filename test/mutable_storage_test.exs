defmodule MutableStorageTest do
  use ExUnit.Case
  doctest MutableStorage

  test "buffer" do
    ref = MutableStorage.buffer_new(1)
    assert MutableStorage.buffer_set_byte(ref, 0, 42) == :ok
    assert MutableStorage.buffer_get_byte(ref, 0) == 42
  end

  test "term" do
    ref = MutableStorage.term_new(42)
    assert MutableStorage.term_get(ref) == 42
    assert MutableStorage.term_set(ref, 43) == :ok
    assert MutableStorage.term_get(ref) == 43
  end
end
