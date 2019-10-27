defmodule MutableBufferTest do
  use ExUnit.Case
  doctest MutableBuffer

  test "greets the world" do
    assert MutableBuffer.hello() == :world
  end

  test "add" do
    assert MutableBuffer.add(1, 2) == {:ok, 3}
  end

  test "buffer" do
    ref = MutableBuffer.new(1)
    MutableBuffer.set_byte(ref, 0, 42)
    assert MutableBuffer.get_byte(ref, 0) == 42
  end
end
