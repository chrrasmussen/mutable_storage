defmodule MutableStorageTest do
  use ExUnit.Case
  # doctest MutableStorage

  @sut_module :mutable_storage

  test "buffer raw size" do
    ref = @sut_module.buffer_new(10)
    assert @sut_module.buffer_raw_size(ref) == 10
  end

  test "buffer bits8 with offset=0" do
    ref = @sut_module.buffer_new(1)
    assert @sut_module.buffer_set_bits8(ref, 0, 42) == :ok
    assert @sut_module.buffer_get_bits8(ref, 0) == 42
  end

  test "buffer bits8 with offset=10" do
    ref = @sut_module.buffer_new(11)
    assert @sut_module.buffer_set_bits8(ref, 10, 37) == :ok
    assert @sut_module.buffer_get_bits8(ref, 10) == 37
  end

  test "buffer int32 with offset=0" do
    ref = @sut_module.buffer_new(8)
    assert @sut_module.buffer_set_int32(ref, 0, 256) == :ok
    assert @sut_module.buffer_get_int32(ref, 0) == 256
  end

  test "buffer int32 with offset=2" do
    ref = @sut_module.buffer_new(10)
    assert @sut_module.buffer_set_int32(ref, 2, 4242) == :ok
    assert @sut_module.buffer_get_int32(ref, 2) == 4242
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

  test "buffer binary with offset=0" do
    ref = @sut_module.buffer_new(4)
    assert @sut_module.buffer_set_binary(ref, 0, "test") == :ok
    assert @sut_module.buffer_get_binary(ref, 0, 4) == "test"
  end

  test "buffer binary with offset=1" do
    ref = @sut_module.buffer_new(6)
    assert @sut_module.buffer_set_binary(ref, 1, "hello") == :ok
    assert @sut_module.buffer_get_binary(ref, 1, 5) == "hello"
  end

  test "buffer binary with unicode characters" do
    ref = @sut_module.buffer_new(2)
    assert @sut_module.buffer_set_binary(ref, 0, "ƒ") == :ok
    assert @sut_module.buffer_get_binary(ref, 0, 2) == "ƒ"
  end

  test "buffer copy" do
    src_ref = @sut_module.buffer_new(10)
    dest_ref = @sut_module.buffer_new(10)
    assert @sut_module.buffer_set_binary(src_ref, 0, "abc") == :ok
    @sut_module.buffer_copy(src_ref, 0, 5, dest_ref, 5)
    assert @sut_module.buffer_get_binary(dest_ref, 5, 3) == "abc"
  end

  test "buffer resize" do
    ref = @sut_module.buffer_new(10)
    assert @sut_module.buffer_raw_size(ref) == 10

    @sut_module.buffer_resize(ref, 5)
    assert @sut_module.buffer_raw_size(ref) == 5

    @sut_module.buffer_resize(ref, 15)
    assert @sut_module.buffer_raw_size(ref) == 15
  end

  test "term" do
    ref = @sut_module.term_new(42)
    assert @sut_module.term_get(ref) == 42
    assert @sut_module.term_set(ref, 43) == :ok
    assert @sut_module.term_get(ref) == 43
  end

  test "term save atomics" do
    ref = @sut_module.term_new(:atomics.new(1, []))
    atomics_ref = @sut_module.term_get(ref)
    assert :atomics.get(atomics_ref, 1) == 0
  end

  test "term save atomics from another process" do
    pid = self()
    spawn(fn() ->
      ref = MutableStorage.term_new(:atomics.new(1, []))
      send(pid, ref)
    end)
    receive do
      ref ->
        atomics_ref = MutableStorage.term_get(ref)
        assert :atomics.get(atomics_ref, 1) == 0
    end
  end
end
