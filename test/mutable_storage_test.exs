defmodule MutableStorageTest do
  use ExUnit.Case
  doctest MutableStorage

  test "buffer" do
    ref = MutableStorage.new(1)
    assert MutableStorage.set_byte(ref, 0, 42) == :ok
    assert MutableStorage.get_byte(ref, 0) == 42
  end

  test "ioref" do
    ref = MutableStorage.ioref_new(42)
    assert MutableStorage.ioref_get(ref) == 42
    assert MutableStorage.ioref_set(ref, 43) == :ok
    assert MutableStorage.ioref_get(ref) == 43
  end
end
