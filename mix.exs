defmodule MutableBuffer.MixProject do
  use Mix.Project

  def project do
    [
      app: :mutable_buffer,
      version: "0.1.0",
      elixir: "~> 1.9",
      compilers: [:rustler] ++ Mix.compilers(),
      start_permanent: Mix.env() == :prod,
      aliases: aliases(),
      deps: deps(),
      rustler_crates: rustler_crates()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp aliases do
    [
      test: ["cmd cd native/mutable_buffer && cargo test", "test"]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.21.0"}
    ]
  end

  defp rustler_crates do
    [
      mutable_buffer: [
        path: "native/mutable_buffer",
        mode: if(Mix.env() == :prod, do: :release, else: :debug)
      ]
    ]
  end
end
