import unittest
from d10 import Srgb

delta = 0.0001


class TestSrgb(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = Srgb(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.666)
        self.assertChannelValue(color.blue, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_with_channels(self):
        color = Srgb(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.red, 0.0)
        self.assertChannelValue(color.green, 0.0)
        self.assertChannelValue(color.blue, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_red(1.0)
        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.0)
        self.assertChannelValue(color.blue, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_green(1.0)
        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 1.0)
        self.assertChannelValue(color.blue, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_blue(1.0)
        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 1.0)
        self.assertChannelValue(color.blue, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 1.0)
        self.assertChannelValue(color.blue, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = Srgb(1.0, 0.666, 0.333, 0.5)

        color = color.to_rgb().to_srgb()

        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.666)
        self.assertChannelValue(color.blue, 0.333)
        self.assertChannelValue(color.alpha, 0.5)
