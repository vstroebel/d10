import unittest
from d10 import Hsv

delta = 0.0001


class TestHsv(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = Hsv(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 0.666)
        self.assertChannelValue(color.value, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_setters(self):
        color = Hsv(0.1, 0.3, 0.5, 0.7)

        self.assertChannelValue(color.hue, 0.1)
        self.assertChannelValue(color.saturation, 0.3)
        self.assertChannelValue(color.value, 0.5)
        self.assertChannelValue(color.alpha, 0.7)

        color.hue = 0.2
        color.saturation = 0.4
        color.value = 0.6
        color.alpha = 0.8

        self.assertChannelValue(color.hue, 0.2)
        self.assertChannelValue(color.saturation, 0.4)
        self.assertChannelValue(color.value, 0.6)
        self.assertChannelValue(color.alpha, 0.8)

    def test_with_channels(self):
        color = Hsv(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.hue, 0.0)
        self.assertChannelValue(color.saturation, 0.0)
        self.assertChannelValue(color.value, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_hue(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 0.0)
        self.assertChannelValue(color.value, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_saturation(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.value, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_value(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.value, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.hue, 1.0)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.value, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = Hsv(0.666, 1.0, 0.333, 0.5)

        color = color.to_rgb().to_hsv()

        self.assertChannelValue(color.hue, 0.666)
        self.assertChannelValue(color.saturation, 1.0)
        self.assertChannelValue(color.value, 0.333)
        self.assertChannelValue(color.alpha, 0.5)
