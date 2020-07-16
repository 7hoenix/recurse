# This file is responsible for configuring your application
# and its dependencies with the aid of the Mix.Config module.
#
# This configuration file is loaded before any dependency and
# is restricted to this project.

# General application configuration
use Mix.Config

config :mmpe,
  namespace: MMPE

# Configures the endpoint
config :mmpe, MMPEWeb.Endpoint,
  url: [host: "localhost"],
  secret_key_base: "MeGq4sBL2N78J+yZKTSC+kJ+AbJUSmZeMh5bqX1Eqcg6w+UfwdxRU5R26SxRIfKx",
  render_errors: [view: MMPEWeb.ErrorView, accepts: ~w(html json), layout: false],
  pubsub_server: MMPE.PubSub,
  live_view: [signing_salt: "L1aUpL/i"]

# Configures Elixir's Logger
config :logger, :console,
  format: "$time $metadata[$level] $message\n",
  metadata: [:request_id]

# Use Jason for JSON parsing in Phoenix
config :phoenix, :json_library, Jason

# Import environment specific config. This must remain at the bottom
# of this file so it overrides the configuration defined above.
import_config "#{Mix.env()}.exs"
