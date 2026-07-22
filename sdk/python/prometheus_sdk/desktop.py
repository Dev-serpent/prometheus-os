"""Desktop SDK - Control the Prometheus desktop environment."""


class Desktop:
    """Interface to the Prometheus desktop shell."""

    def open_file(self, path: str):
        """Open a file in the default application."""
        pass

    def open_url(self, url: str):
        """Open a URL in the browser."""
        pass

    def send_notification(self, title: str, body: str):
        """Send a desktop notification."""
        pass

    def set_clipboard(self, text: str):
        """Set clipboard contents."""
        pass

    def get_clipboard(self) -> str:
        """Get clipboard contents."""
        return ""

    def switch_workspace(self, index: int):
        """Switch to a workspace by index."""
        pass
