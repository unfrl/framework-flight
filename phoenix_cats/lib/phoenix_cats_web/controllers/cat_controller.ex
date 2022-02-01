defmodule PhoenixCatsWeb.CatController do
  use PhoenixCatsWeb, :controller

  alias PhoenixCats.Cat

  def index(conn, _params) do
    json(conn, %{cats: ["meow"]})
  end

  def show(conn, %{"id" => id}) do
    json(conn, %{cat: id})
  end
end
