defmodule PhoenixCats.AnimalsTest do
  use PhoenixCats.DataCase

  alias PhoenixCats.Animals

  describe "cats" do
    alias PhoenixCats.Animals.Cat

    import PhoenixCats.AnimalsFixtures

    @invalid_attrs %{color: nil, name: nil}

    test "list_cats/0 returns all cats" do
      cat = cat_fixture()
      assert Animals.list_cats() == [cat]
    end

    test "get_cat!/1 returns the cat with given id" do
      cat = cat_fixture()
      assert Animals.get_cat!(cat.id) == cat
    end

    test "create_cat/1 with valid data creates a cat" do
      valid_attrs = %{color: "some color", name: "some name"}

      assert {:ok, %Cat{} = cat} = Animals.create_cat(valid_attrs)
      assert cat.color == "some color"
      assert cat.name == "some name"
    end

    test "create_cat/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Animals.create_cat(@invalid_attrs)
    end

    test "update_cat/2 with valid data updates the cat" do
      cat = cat_fixture()
      update_attrs = %{color: "some updated color", name: "some updated name"}

      assert {:ok, %Cat{} = cat} = Animals.update_cat(cat, update_attrs)
      assert cat.color == "some updated color"
      assert cat.name == "some updated name"
    end

    test "update_cat/2 with invalid data returns error changeset" do
      cat = cat_fixture()
      assert {:error, %Ecto.Changeset{}} = Animals.update_cat(cat, @invalid_attrs)
      assert cat == Animals.get_cat!(cat.id)
    end

    test "delete_cat/1 deletes the cat" do
      cat = cat_fixture()
      assert {:ok, %Cat{}} = Animals.delete_cat(cat)
      assert_raise Ecto.NoResultsError, fn -> Animals.get_cat!(cat.id) end
    end

    test "change_cat/1 returns a cat changeset" do
      cat = cat_fixture()
      assert %Ecto.Changeset{} = Animals.change_cat(cat)
    end
  end
end
