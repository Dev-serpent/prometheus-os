"""Prometheus AI SDK - Interact with the Prometheus AI Core."""

import json
import socket


class AI:
    """Interface to the Prometheus AI engine."""

    def __init__(self, socket_path="/run/prometheus/ai.sock"):
        self._socket_path = socket_path

    def query(self, text: str, context: str = None) -> dict:
        """Send a query to Prometheus AI."""
        payload = {"type": "query", "text": text}
        if context:
            payload["context"] = context
        return self._send(payload)

    def execute(self, action: str) -> dict:
        """Execute an action through Prometheus."""
        payload = {"type": "execute", "action": action}
        return self._send(payload)

    def analyze_screen(self) -> dict:
        """Ask Prometheus to analyze the current screen contents."""
        payload = {"type": "analyze_screen"}
        return self._send(payload)

    def _send(self, payload: dict) -> dict:
        try:
            with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as sock:
                sock.connect(self._socket_path)
                sock.send(json.dumps(payload).encode())
                response = sock.recv(65536).decode()
                return json.loads(response)
        except (ConnectionRefusedError, FileNotFoundError):
            return {"error": "Prometheus AI is not running"}
