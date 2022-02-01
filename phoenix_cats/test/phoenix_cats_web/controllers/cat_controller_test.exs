defmodule PhoenixCatsWeb.CatControllerTest do
  use PhoenixCatsWeb.ConnCase

  import PhoenixCats.AnimalsFixtures

  alias PhoenixCats.Animals.Cat

  @create_attrs %{
    color: "some color",
    name: "some name"
  }
  @update_attrs %{
    color: "some updated color",
    name: "some updated name"
  }
  @invalid_attrs %{color: nil, name: nil}

  setup %{conn: conn} do
    {:ok, conn: put_req_header(conn, "accept", "application/json")}
  end

  describe "index" do
    test "lists all cats", %{conn: conn} do
      conn = get(conn, Routes.cat_path(conn, :index))
      assert json_response(conn, 200)["data"] == []
    end
  end

  describe "create cat" do
    test "renders cat when data is valid", %{conn: conn} do
      conn = post(conn, Routes.cat_path(conn, :create), cat: @create_attrs)
      assert %{"id" => id} = json_response(conn, 201)["data"]

      conn = get(conn, Routes.cat_path(conn, :show, id))

      assert %{
               "id" => ^id,
               "color" => "some color",
               "name" => "some name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn} do
      conn = post(conn, Routes.cat_path(conn, :create), cat: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "update cat" do
    setup [:create_cat]

    test "renders cat when data is valid", %{conn: conn, cat: %Cat{id: id} = cat} do
      conn = put(conn, Routes.cat_path(conn, :update, cat), cat: @update_attrs)
      assert %{"id" => ^id} = json_response(conn, 200)["data"]

      conn = get(conn, Routes.cat_path(conn, :show, id))

      assert %{
               "id" => ^id,
               "color" => "some updated color",
               "name" => "some updated name"
             } = json_response(conn, 200)["data"]
    end

    test "renders errors when data is invalid", %{conn: conn, cat: cat} do
      conn = put(conn, Routes.cat_path(conn, :update, cat), cat: @invalid_attrs)
      assert json_response(conn, 422)["errors"] != %{}
    end
  end

  describe "delete cat" do
    setup [:create_cat]

    test "deletes chosen cat", %{conn: conn, cat: cat} do
      conn = delete(conn, Routes.cat_path(conn, :delete, cat))
      assert response(conn, 204)

      assert_error_sent 404, fn ->
        get(conn, Routes.cat_path(conn, :show, cat))
      end
    end
  end

  defp create_cat(_) do
    cat = cat_fixture()
    %{cat: cat}
  end
end
