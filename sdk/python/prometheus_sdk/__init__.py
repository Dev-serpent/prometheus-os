"""Prometheus OS Python SDK

Build AI-powered applications for Prometheus OS.

Usage:
    from prometheus_sdk import PrometheusAI

    ai = PrometheusAI()
    response = ai.query("What is on my screen?")
"""

from .ai import AI
from .desktop import Desktop
from .system import System


class PrometheusSDK:
    def __init__(self):
        self.ai = AI()
        self.desktop = Desktop()
        self.system = System()

    def version(self):
        return "0.1.0"


__all__ = ["PrometheusSDK"]
