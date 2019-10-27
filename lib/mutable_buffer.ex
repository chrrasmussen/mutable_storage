defmodule MutableBuffer do
  use Rustler, otp_app: :mutable_buffer, crate: :mutable_buffer

  @moduledoc """
  Documentation for MutableBuffer.
  """

  @doc """
  Hello world.

  ## Examples

      iex> MutableBuffer.hello()
      :world

  """
  def hello do
    :world
  end

  def add(_x, _y), do: :erlang.nif_error(:nif_not_loaded)

  def new(_x), do: :erlang.nif_error(:nif_not_loaded)
  def get_byte(_x, _y), do: :erlang.nif_error(:nif_not_loaded)
  def set_byte(_x, _y, _z), do: :erlang.nif_error(:nif_not_loaded)
end
