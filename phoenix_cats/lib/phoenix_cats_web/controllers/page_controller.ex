defmodule PhoenixCatsWeb.PageController do
  use PhoenixCatsWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html")
  end
end
