defmodule MMPEWeb.PageLive do
  use MMPEWeb, :live_view

  @impl true
  def render(assigns) do
    ~L"""
    <div class="grid" id="grid">
      <%= for y <- 0..@height - 1, x <- 0..@width-1 do %>
        <div id="<%= x %>-<%= y %>" class="grid-cell white">
        </div>
      <% end %>
    </div>
    """
  end

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket
      |> assign(:grid, %{})
      |> assign(:width, 30)
      |> assign(:height, 24)
    }
  end

  def handle_event("inc_counter", _session, socket) do
    {:noreply, socket
      |> assign(:counter, socket.assigns.counter + 1)
    }
  end

end