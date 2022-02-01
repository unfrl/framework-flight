defmodule PhoenixCats.AnimalsFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `PhoenixCats.Animals` context.
  """

  @doc """
  Generate a cat.
  """
  def cat_fixture(attrs \\ %{}) do
    {:ok, cat} =
      attrs
      |> Enum.into(%{
        color: "some color",
        name: "some name"
      })
      |> PhoenixCats.Animals.create_cat()

    cat
  end
end
