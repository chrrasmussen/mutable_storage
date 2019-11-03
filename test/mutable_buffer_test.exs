defmodule MutableBufferTest do
  use ExUnit.Case
  doctest MutableBuffer

  test "buffer" do
    ref = MutableBuffer.new(1)
    assert MutableBuffer.set_byte(ref, 0, 42) == :ok
    assert MutableBuffer.get_byte(ref, 0) == 42
  end

  test "ioref" do
    ref = MutableBuffer.ioref_new(42)
    assert MutableBuffer.ioref_get(ref) == 42
    assert MutableBuffer.ioref_set(ref, 43) == :ok
    assert MutableBuffer.ioref_get(ref) == 43
  end
end
