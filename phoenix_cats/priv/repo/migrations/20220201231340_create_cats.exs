defmodule PhoenixCats.Repo.Migrations.CreateCats do
  use Ecto.Migration

  def change do
    create table(:cats) do
      add :name, :string
      add :color, :string

      timestamps()
    end
  end
end
