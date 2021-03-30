import unittest
from d10 import Xyz

delta = 0.0001


class TestXyz(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = Xyz(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.x, 1.0)
        self.assertChannelValue(color.y, 0.666)
        self.assertChannelValue(color.z, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_setters(self):
        color = Xyz(0.1, 0.3, 0.5, 0.7)

        self.assertChannelValue(color.x, 0.1)
        self.assertChannelValue(color.y, 0.3)
        self.assertChannelValue(color.z, 0.5)
        self.assertChannelValue(color.alpha, 0.7)

        color.x = 0.2
        color.y = 0.4
        color.z = 0.6
        color.alpha = 0.8

        self.assertChannelValue(color.x, 0.2)
        self.assertChannelValue(color.y, 0.4)
        self.assertChannelValue(color.z, 0.6)
        self.assertChannelValue(color.alpha, 0.8)

    def test_with_channels(self):
        color = Xyz(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.x, 0.0)
        self.assertChannelValue(color.y, 0.0)
        self.assertChannelValue(color.z, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_x(1.0)
        self.assertChannelValue(color.x, 1.0)
        self.assertChannelValue(color.y, 0.0)
        self.assertChannelValue(color.z, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_y(1.0)
        self.assertChannelValue(color.x, 1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.z, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_z(1.0)
        self.assertChannelValue(color.x, 1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.z, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.x, 1.0)
        self.assertChannelValue(color.y, 1.0)
        self.assertChannelValue(color.z, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = Xyz(0.52760778, 0.3811918, 0.24823388, 0.5)

        color = color.to_rgb().to_xyz()

        self.assertChannelValue(color.x, 0.52760778)
        self.assertChannelValue(color.y, 0.3811918)
        self.assertChannelValue(color.z, 0.24823388)
        self.assertChannelValue(color.alpha, 0.5)
