import unittest
from d10 import RGB

delta = 0.0001


class TestRGB(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = RGB(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.666)
        self.assertChannelValue(color.blue, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_with_channels(self):
        color = RGB(0.0, 0.0, 0.0, 0.0)
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

    def test_equals(self):
        color1 = RGB(1.0, 0.666, 0.333, 0.5)
        color2 = RGB(1.0, 0.666, 0.333, 0.5)
        color3 = RGB(0.0, 0.666, 0.333, 0.5)

        self.assertEqual(color1, color2)
        self.assertNotEqual(color1, color3)

    def test_is_grayscale(self):
        self.assertTrue(RGB(0.5, 0.5, 0.5).is_grayscale())
        self.assertFalse(RGB(1.0, 0.5, 0.5).is_grayscale())

    def test_to_gray(self):
        color = RGB(0.0, 1.0, 0.0).to_gray()
        self.assertChannelValue(color.red, 0.715_158)

        color = RGB(0.0, 1.0, 0.0).to_gray('rec601luma')
        self.assertChannelValue(color.red, 0.586_811)

    def test_invert(self):
        color = RGB(1.0, 0.666, 0.333).invert()

        self.assertChannelValue(color.red, 0.0)
        self.assertChannelValue(color.green, 1.0 - 0.666)
        self.assertChannelValue(color.blue, 1.0 - 0.333)
        self.assertChannelValue(color.alpha, 1.0)

    def test_difference(self):
        color1 = RGB(1.0, 0.666, 0.333)
        color2 = RGB(0.0, 1.0, 1.0)

        color = color1.difference(color2)

        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 1.0 - 0.666)
        self.assertChannelValue(color.blue, 1.0 - 0.333)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_gamma(self):
        color = RGB(1.0, 0.666, 0.333).with_gamma(1.5)

        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.762_633)
        self.assertChannelValue(color.blue, 0.480_429)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_level(self):
        color = RGB(1.0, 0.666, 0.333).with_level(-0.1, 0.9, 1.5)

        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.837_180)
        self.assertChannelValue(color.blue, 0.572_346)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_brightness(self):
        color = RGB(1.0, 0.666, 0.333).with_brightness(0.1)

        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.766)
        self.assertChannelValue(color.blue, 0.433)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_saturation(self):
        color = RGB(1.0, 0.666, 0.333).with_saturation(0.5)

        self.assertChannelValue(color.red, 0.833_250)
        self.assertChannelValue(color.green, 0.666_25)
        self.assertChannelValue(color.blue, 0.499_75)
        self.assertChannelValue(color.alpha, 1.0)

    def test_stretch_saturation(self):
        color = RGB(1.0, 0.666, 0.333).stretch_saturation(0.5)

        self.assertChannelValue(color.red, 0.916_6)
        self.assertChannelValue(color.green, 0.666_1)
        self.assertChannelValue(color.blue, 0.416_4)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_lightness(self):
        color = RGB(1.0, 0.666, 0.333).with_lightness(0.5)

        self.assertChannelValue(color.red, 0.666_5)
        self.assertChannelValue(color.green, 0.332_8)
        self.assertChannelValue(color.blue, 0.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_hue_rotate(self):
        color = RGB(1.0, 0.666666, 0.333333).with_hue_rotate(180)

        self.assertChannelValue(color.red, 0.333333)
        self.assertChannelValue(color.green, 0.66666)
        self.assertChannelValue(color.blue, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_contrast(self):
        color = RGB(1.0, 0.666, 0.333).with_contrast(0.5)

        self.assertChannelValue(color.red, 0.75)
        self.assertChannelValue(color.green, 0.583)
        self.assertChannelValue(color.blue, 0.416_5)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_brightness_contrast(self):
        color = RGB(1.0, 0.666, 0.333).with_brightness_contrast(0.1, 0.5)

        self.assertChannelValue(color.red, 0.8)
        self.assertChannelValue(color.green, 0.633)
        self.assertChannelValue(color.blue, 0.466_5)
        self.assertChannelValue(color.alpha, 1.0)

    def test_alpha_blend(self):
        color1 = RGB(1.0, 0.666, 0.333)
        color2 = RGB(0.5, 1.0, 1.0, 0.5)

        color = color1.alpha_blend(color2)

        self.assertChannelValue(color.red, (1.0 + 0.5) / 2.0)
        self.assertChannelValue(color.green, (0.666 + 1.0) / 2.0)
        self.assertChannelValue(color.blue, (0.333 + 1.0) / 2.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_with_vibrance(self):
        color1 = RGB(1.0, 0.666, 0.333)
        color2 = color1.with_vibrance(0.5)
        color3 = color1.with_vibrance(1.5)

        # As vibrance might change in the future only test if
        # it changed something nd different factors produce different results
        self.assertNotEqual(color1, color2)
        self.assertNotEqual(color2, color3)

    def test_modulate(self):
        color = RGB(1.0, 0.666, 0.333).modulate(1.1, 1.2, 1.3)

        self.assertChannelValue(color.red, 1.0)
        self.assertChannelValue(color.green, 0.879_6)
        self.assertChannelValue(color.blue, 0.732_9)
        self.assertChannelValue(color.alpha, 1.0)

    def test_min(self):
        color = RGB(0.8, 0.666, 0.333)
        self.assertChannelValue(color.min(), 0.333)

    def test_max(self):
        color = RGB(0.8, 0.666, 0.333)
        self.assertChannelValue(color.max(), 0.8)

    def test_mod_color_channels(self):
        color1 = RGB(0.8, 0.5, 0.3, 0.5)
        color2 = RGB(0.9, 0.6, 0.4, 0.5)

        color3 = color1.map_color_channels(lambda v: v + 0.1)

        self.assertEqual(color2, color3)
