import unittest
from d10 import Lab
from d10 import Lch

delta = 0.0001


class TestLab(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = Lab(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.a, 0.666)
        self.assertChannelValue(color.b, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_setters(self):
        color = Lab(0.1, 0.3, 0.5, 0.7)

        self.assertChannelValue(color.l, 0.1)
        self.assertChannelValue(color.a, 0.3)
        self.assertChannelValue(color.b, 0.5)
        self.assertChannelValue(color.alpha, 0.7)

        color.l = 0.2
        color.a = 0.4
        color.b = 0.6
        color.alpha = 0.8

        self.assertChannelValue(color.l, 0.2)
        self.assertChannelValue(color.a, 0.4)
        self.assertChannelValue(color.b, 0.6)
        self.assertChannelValue(color.alpha, 0.8)

    def test_with_channels(self):
        color = Lab(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.l, 0.0)
        self.assertChannelValue(color.a, 0.0)
        self.assertChannelValue(color.b, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_l(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.a, 0.0)
        self.assertChannelValue(color.b, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_a(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.a, 1.0)
        self.assertChannelValue(color.b, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_b(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.a, 1.0)
        self.assertChannelValue(color.b, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.a, 1.0)
        self.assertChannelValue(color.b, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = Lab(0.5, 0.6, 0.4, 0.1)

        color = color.to_rgb().to_lab()

        self.assertChannelValue(color.l, 0.5)
        self.assertChannelValue(color.a, 0.6)
        self.assertChannelValue(color.b, 0.4)
        self.assertChannelValue(color.alpha, 0.1)

    def test_lab_types(self):
        self.assertEqual(Lab(1, 1, 1, 1).type_name, "lab<D65,O2>")
        self.assertEqual(Lab(1, 1, 1, 1, 'D65', '2').type_name, "lab<D65,O2>")
        self.assertEqual(Lab(1, 1, 1, 1, 'D65', '10').type_name, "lab<D65,O10>")
        self.assertEqual(Lab(1, 1, 1, 1, 'D50', '2').type_name, "lab<D50,O2>")
        self.assertEqual(Lab(1, 1, 1, 1, 'D50', '10').type_name, "lab<D50,O10>")
        self.assertEqual(Lab(1, 1, 1, 1, 'E', '2').type_name, "lab<E,O2>")
        self.assertEqual(Lab(1, 1, 1, 1, 'E', '10').type_name, "lab<E,O10>")


class TestLch(unittest.TestCase):

    def assertChannelValue(self, first, second):
        self.assertAlmostEqual(first, second, delta=delta)

    def test_new(self):
        color = Lch(1.0, 0.666, 0.333, 0.5)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.c, 0.666)
        self.assertChannelValue(color.h, 0.333)
        self.assertChannelValue(color.alpha, 0.5)

    def test_setters(self):
        color = Lch(0.1, 0.3, 0.5, 0.7)

        self.assertChannelValue(color.l, 0.1)
        self.assertChannelValue(color.c, 0.3)
        self.assertChannelValue(color.h, 0.5)
        self.assertChannelValue(color.alpha, 0.7)

        color.l = 0.2
        color.c = 0.4
        color.h = 0.6
        color.alpha = 0.8

        self.assertChannelValue(color.l, 0.2)
        self.assertChannelValue(color.c, 0.4)
        self.assertChannelValue(color.h, 0.6)
        self.assertChannelValue(color.alpha, 0.8)

    def test_with_channels(self):
        color = Lch(0.0, 0.0, 0.0, 0.0)
        self.assertChannelValue(color.l, 0.0)
        self.assertChannelValue(color.c, 0.0)
        self.assertChannelValue(color.h, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_l(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.c, 0.0)
        self.assertChannelValue(color.h, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_c(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.c, 1.0)
        self.assertChannelValue(color.h, 0.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_h(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.c, 1.0)
        self.assertChannelValue(color.h, 1.0)
        self.assertChannelValue(color.alpha, 0.0)

        color = color.with_alpha(1.0)
        self.assertChannelValue(color.l, 1.0)
        self.assertChannelValue(color.c, 1.0)
        self.assertChannelValue(color.h, 1.0)
        self.assertChannelValue(color.alpha, 1.0)

    def test_conversion(self):
        color = Lch(0.5, 0.6, 0.4, 0.1)

        color = color.to_rgb().to_lch()

        self.assertChannelValue(color.l, 0.5)
        self.assertChannelValue(color.c, 0.6)
        self.assertChannelValue(color.h, 0.4)
        self.assertChannelValue(color.alpha, 0.1)

    def test_lch_types(self):
        self.assertEqual(Lch(1, 1, 1, 1).type_name, "lch<D65,O2>")
        self.assertEqual(Lch(1, 1, 1, 1, 'D65', '2').type_name, "lch<D65,O2>")
        self.assertEqual(Lch(1, 1, 1, 1, 'D65', '10').type_name, "lch<D65,O10>")
        self.assertEqual(Lch(1, 1, 1, 1, 'D50', '2').type_name, "lch<D50,O2>")
        self.assertEqual(Lch(1, 1, 1, 1, 'D50', '10').type_name, "lch<D50,O10>")
        self.assertEqual(Lch(1, 1, 1, 1, 'E', '2').type_name, "lch<E,O2>")
        self.assertEqual(Lch(1, 1, 1, 1, 'E', '10').type_name, "lch<E,O10>")
