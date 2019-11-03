defmodule MutableBufferTest do
  use ExUnit.Case
  doctest MutableBuffer

  test "buffer" do
    ref = MutableBuffer.new(1)
    MutableBuffer.set_byte(ref, 0, 42)
    assert MutableBuffer.get_byte(ref, 0) == 42
  end
end
