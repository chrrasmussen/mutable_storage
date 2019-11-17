defmodule MutableStorageTest do
  use ExUnit.Case
  doctest MutableStorage

  @sut_module MutableStorage

  test "term" do
    ref = @sut_module.term_new(42)
    assert @sut_module.term_get(ref) == 42
    assert @sut_module.term_set(ref, 43) == :ok
    assert @sut_module.term_get(ref) == 43
  end

  test "buffer byte with offset=0" do
    ref = @sut_module.buffer_new(1)
    assert @sut_module.buffer_set_byte(ref, 0, 42) == :ok
    assert @sut_module.buffer_get_byte(ref, 0) == 42
  end

  test "buffer byte with offset=10" do
    ref = @sut_module.buffer_new(11)
    assert @sut_module.buffer_set_byte(ref, 10, 37) == :ok
    assert @sut_module.buffer_get_byte(ref, 10) == 37
  end

  test "buffer int with offset=0" do
    ref = @sut_module.buffer_new(8)
    assert @sut_module.buffer_set_int(ref, 0, 256) == :ok
    assert @sut_module.buffer_get_int(ref, 0) == 256
  end

  test "buffer int with offset=2" do
    ref = @sut_module.buffer_new(10)
    assert @sut_module.buffer_set_int(ref, 2, 4242) == :ok
    assert @sut_module.buffer_get_int(ref, 2) == 4242
  end

  test "buffer double with offset=0" do
    ref = @sut_module.buffer_new(8)
    assert @sut_module.buffer_set_double(ref, 0, 256.256) == :ok
    assert @sut_module.buffer_get_double(ref, 0) == 256.256
  end

  test "buffer double with offset=2" do
    ref = @sut_module.buffer_new(10)
    assert @sut_module.buffer_set_double(ref, 2, 42.42) == :ok
    assert @sut_module.buffer_get_double(ref, 2) == 42.42
  end
end
