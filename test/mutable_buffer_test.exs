defmodule MutableBufferTest do
  use ExUnit.Case
  doctest MutableBuffer

  test "greets the world" do
    assert MutableBuffer.hello() == :world
  end

  test "add" do
    assert MutableBuffer.add(1, 2) == {:ok, 3}
  end
end
