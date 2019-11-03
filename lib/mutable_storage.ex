defmodule MutableStorage do
  use Rustler, otp_app: :mutable_storage, crate: :mutable_storage

  @moduledoc """
  Documentation for MutableStorage.
  """

  def buffer_new(_x), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_get_byte(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def buffer_set_byte(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)

  def term_new(_x), do: :erlang.nif_error(:nif_not_loaded)
  def term_get(_x), do: :erlang.nif_error(:nif_not_loaded)
  def term_set(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
end
