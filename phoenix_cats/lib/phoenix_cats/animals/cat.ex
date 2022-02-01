defmodule PhoenixCats.Animals.Cat do
  use Ecto.Schema
  import Ecto.Changeset

  schema "cats" do
    field :color, :string
    field :name, :string

    timestamps()
  end

  @doc false
  def changeset(cat, attrs) do
    cat
    |> cast(attrs, [:name, :color])
    |> validate_required([:name, :color])
  end
end
