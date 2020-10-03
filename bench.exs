small_value = "1234567890"
big_value = List.duplicate(small_value, 1_000)

iterations = 1_000

Benchee.run(
  %{
    "MutableStorage (Read/write)" => fn value ->
      ref = MutableStorage.term_new(value)

      (1..iterations)
      |> Enum.map(fn _ ->
        _ = MutableStorage.term_get(ref)
        MutableStorage.term_set(ref, value)
      end)
    end,
    "MutableStorage (Only read)" => fn value ->
      ref = MutableStorage.term_new(value)

      (1..iterations)
      |> Enum.map(fn _ ->
        _ = MutableStorage.term_get(ref)
      end)
    end,
    "MutableStorage (Only write)" => fn value ->
      ref = MutableStorage.term_new(value)

      (1..iterations)
      |> Enum.map(fn _ ->
        MutableStorage.term_set(ref, value)
      end)
    end,
    "Process dict (Read/write)" => fn value ->
      ref = make_ref()

      (1..iterations)
      |> Enum.map(fn _ ->
        _ = :erlang.get(ref)
        :erlang.put(ref, value)
      end)
    end
  },
  time: 10,
  memory_time: 2,
  inputs: %{
    "Small value" => small_value,
    "Big value" => big_value
  }
)
