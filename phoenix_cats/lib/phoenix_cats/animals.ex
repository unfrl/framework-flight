defmodule PhoenixCats.Animals do
  @moduledoc """
  The Animals context.
  """

  import Ecto.Query, warn: false
  alias PhoenixCats.Repo

  alias PhoenixCats.Animals.Cat

  @doc """
  Returns the list of cats.

  ## Examples

      iex> list_cats()
      [%Cat{}, ...]

  """
  def list_cats do
    Repo.all(Cat)
  end

  @doc """
  Gets a single cat.

  Raises `Ecto.NoResultsError` if the Cat does not exist.

  ## Examples

      iex> get_cat!(123)
      %Cat{}

      iex> get_cat!(456)
      ** (Ecto.NoResultsError)

  """
  def get_cat!(id), do: Repo.get!(Cat, id)

  @doc """
  Creates a cat.

  ## Examples

      iex> create_cat(%{field: value})
      {:ok, %Cat{}}

      iex> create_cat(%{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def create_cat(attrs \\ %{}) do
    %Cat{}
    |> Cat.changeset(attrs)
    |> Repo.insert()
  end

  @doc """
  Updates a cat.

  ## Examples

      iex> update_cat(cat, %{field: new_value})
      {:ok, %Cat{}}

      iex> update_cat(cat, %{field: bad_value})
      {:error, %Ecto.Changeset{}}

  """
  def update_cat(%Cat{} = cat, attrs) do
    cat
    |> Cat.changeset(attrs)
    |> Repo.update()
  end

  @doc """
  Deletes a cat.

  ## Examples

      iex> delete_cat(cat)
      {:ok, %Cat{}}

      iex> delete_cat(cat)
      {:error, %Ecto.Changeset{}}

  """
  def delete_cat(%Cat{} = cat) do
    Repo.delete(cat)
  end

  @doc """
  Returns an `%Ecto.Changeset{}` for tracking cat changes.

  ## Examples

      iex> change_cat(cat)
      %Ecto.Changeset{data: %Cat{}}

  """
  def change_cat(%Cat{} = cat, attrs \\ %{}) do
    Cat.changeset(cat, attrs)
  end
end
