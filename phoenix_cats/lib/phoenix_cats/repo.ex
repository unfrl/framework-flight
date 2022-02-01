defmodule PhoenixCats.Repo do
  use Ecto.Repo,
    otp_app: :phoenix_cats,
    adapter: Ecto.Adapters.Postgres
end
