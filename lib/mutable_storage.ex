defmodule MutableStorage do
  use Rustler, otp_app: :mutable_storage, crate: :mutable_storage

  @moduledoc """
  Documentation for MutableStorage.
  """

  def buffer_new(_x), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_raw_size(_x), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_bits8(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_bits8(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_bits16(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_bits16(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_bits32(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_bits32(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_bits64(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_bits64(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_int32(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_int32(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_int64(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_int64(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_double(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_double(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_binary(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_binary(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)

  def term_new(_x), do: :erlang.nif_error(:nif_not_loaded)
  def term_get(_x), do: :erlang.nif_error(:nif_not_loaded)
  def term_set(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
end
