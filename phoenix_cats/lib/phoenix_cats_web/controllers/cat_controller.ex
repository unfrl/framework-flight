defmodule PhoenixCatsWeb.CatController do
  use PhoenixCatsWeb, :controller

  alias PhoenixCats.Animals
  alias PhoenixCats.Animals.Cat

  action_fallback PhoenixCatsWeb.FallbackController

  def index(conn, _params) do
    cats = Animals.list_cats()
    render(conn, "index.json", cats: cats)
  end

  def create(conn, %{"cat" => cat_params}) do
    with {:ok, %Cat{} = cat} <- Animals.create_cat(cat_params) do
      conn
      |> put_status(:created)
      |> put_resp_header("location", Routes.cat_path(conn, :show, cat))
      |> render("show.json", cat: cat)
    end
  end

  def show(conn, %{"id" => id}) do
    cat = Animals.get_cat!(id)
    render(conn, "show.json", cat: cat)
  end

  def update(conn, %{"id" => id, "cat" => cat_params}) do
    cat = Animals.get_cat!(id)

    with {:ok, %Cat{} = cat} <- Animals.update_cat(cat, cat_params) do
      render(conn, "show.json", cat: cat)
    end
  end

  def delete(conn, %{"id" => id}) do
    cat = Animals.get_cat!(id)

    with {:ok, %Cat{}} <- Animals.delete_cat(cat) do
      send_resp(conn, :no_content, "")
    end
  end
end
