defmodule MMPEWeb.PageLive do
  use MMPEWeb, :live_view

  @impl true
  def render(assigns) do
    ~L"""
    <a href="#" phx-click="inc_counter">Increment</a>
    <p>Counter: <%= @counter %></p>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    socket = assign(socket, :counter, 0)
    {:ok, socket}
  end

  def handle_event("inc_counter", _session, socket) do
    {:noreply, socket
      |> assign(:counter, socket.assigns.counter + 1)
    }
  end

end