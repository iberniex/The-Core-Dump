"""
Getter and Setter Module.

Learning on getter and setter modules.
"""


class Label:
    """Label Class."""

    def __init__(self, text, font):
        self._text = text
        self._font = font

    def get_text(self):
        """Getter function for text."""
        return self._text

    def set_text(self, value):
        """Setter function for text."""
        self._text = value

    def get_font(self):
        """Getter function for font."""
        return self._font

    def set_font(self, value):
        """Setter function for font."""
        self._font = value
