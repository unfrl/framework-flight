defmodule PhoenixCatsWeb.CatController do
  use PhoenixCatsWeb, :controller

  def index(conn, _params) do
    json(conn, %{cats: ["meow"]})
  end
end
