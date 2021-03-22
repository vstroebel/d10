import unittest
from d10 import YUV

delta = 0.0001

class TestYUV(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = YUV(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.u, 0.666)
        self.assertChannelValue(color.v, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_with_channels(self):
        color = YUV(0.0, 0.0, 0.0, 0.0)
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
        color = YUV(0.114, 0.43601036, -0.10001026, 0.5)

        color = color.to_rgb().to_yuv()

        self.assertChannelValue(color.y, 0.114)
        self.assertChannelValue(color.u, 0.43601036)
        self.assertChannelValue(color.v, -0.10001026)
        self.assertChannelValue(color.alpha, 0.5)
