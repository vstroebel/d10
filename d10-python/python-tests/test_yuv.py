import unittest
from d10 import Yuv

delta = 0.0001


class TestYuv(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = Yuv(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.u, 0.666)
        self.assertChannelValue(color.v, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_setters(self):
        color = Yuv(0.1, 0.3, 0.5, 0.7)

        self.assertChannelValue(color.y, 0.1)
        self.assertChannelValue(color.u, 0.3)
        self.assertChannelValue(color.v, 0.5)
        self.assertChannelValue(color.alpha, 0.7)

        color.y = 0.2
        color.u = 0.4
        color.v = 0.6
        color.alpha = 0.8

        self.assertChannelValue(color.y, 0.2)
        self.assertChannelValue(color.u, 0.4)
        self.assertChannelValue(color.v, 0.6)
        self.assertChannelValue(color.alpha, 0.8)

    def test_with_channels(self):
        color = Yuv(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.y, 0.0)
        self.assertChannelValue(color.u, 0.0)
        self.assertChannelValue(color.v, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_y(1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.u, 0.0)
        self.assertChannelValue(color.v, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_u(1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.u, 1.0)
        self.assertChannelValue(color.v, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_v(1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.u, 1.0)
        self.assertChannelValue(color.v, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.u, 1.0)
        self.assertChannelValue(color.v, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = Yuv(0.114, 0.43601036, -0.10001026, 0.5)

        color = color.to_rgb().to_yuv()

        self.assertChannelValue(color.y, 0.114)
        self.assertChannelValue(color.u, 0.43601036)
        self.assertChannelValue(color.v, -0.10001026)
        self.assertChannelValue(color.alpha, 0.5)
