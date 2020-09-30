defmodule IORefTest do
  def works() do
    ref = MutableStorage.term_new(:atomics.new(1, []))
    atomics_ref = MutableStorage.term_get(ref)
    IO.puts(:atomics.get(atomics_ref, 1))
  end

  def fails() do
    pid = self()
    spawn(fn() ->
      ref = MutableStorage.term_new(:atomics.new(1, []))
      send(pid, ref)
    end)
    receive do
      ref ->
        atomics_ref = MutableStorage.term_get(ref)
        IO.puts(:atomics.get(atomics_ref, 1))
    end
  end
end

IORefTest.works()
# Prints:
# 0

IORefTest.fails()
# Causes error:
# ** (ArgumentError) argument error
#     :atomics.get(#Reference<0.3809902068.2765225986.183736>, 1)
#     ioref.exs:17: IORefTest.not_working/0
#     (elixir 1.10.3) lib/code.ex:926: Code.require_file/2
#     (mix 1.10.3) lib/mix/tasks/run.ex:145: Mix.Tasks.Run.run/5
