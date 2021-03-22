import unittest
from d10 import HSL

delta = 0.0001


class TestHSL(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = HSL(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 0.666)
        self.assertChannelValue(color.lightness, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_with_channels(self):
        color = HSL(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.hue, 0.0)
        self.assertChannelValue(color.saturation, 0.0)
        self.assertChannelValue(color.lightness, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_hue(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 0.0)
        self.assertChannelValue(color.lightness, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_saturation(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.lightness, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_lightness(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.lightness, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.lightness, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = HSL(0.666, 1.0, 0.333, 0.5)

        color = color.to_rgb().to_hsl()

        self.assertChannelValue(color.hue, 0.666)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.lightness, 0.333)
        self.assertChannelValue(color.alpha, 0.5)
