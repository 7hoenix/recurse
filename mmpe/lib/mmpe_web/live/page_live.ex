defmodule MMPEWeb.PageLive do
  use MMPEWeb, :live_view

  @impl true
  def render(assigns) do
    ~L"""
    <p>Hello world</p>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end
end