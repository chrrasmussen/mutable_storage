defmodule MutableStorageTest do
  use ExUnit.Case
  doctest MutableStorage

  @sut_module MutableStorage

  test "buffer" do
    ref = @sut_module.buffer_new(1)
    assert @sut_module.buffer_set_byte(ref, 0, 42) == :ok
    assert @sut_module.buffer_get_byte(ref, 0) == 42
  end

  test "term" do
    ref = @sut_module.term_new(42)
    assert @sut_module.term_get(ref) == 42
    assert @sut_module.term_set(ref, 43) == :ok
    assert @sut_module.term_get(ref) == 43
  end
end
