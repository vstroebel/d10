import unittest
import numpy as np

from d10 import Image, RGB


class TestImage(unittest.TestCase):

    def test_new(self):
        image = Image(4, 7)

        self.assertEqual(image.width, 4)
        self.assertEqual(image.height, 7)

    def test_from_list(self):
        image = Image.from_list(2, 3, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(1.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
        ])

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

        self.assertEqual(image.get_pixel(0, 0), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 2), RGB(1.0, 0.0, 1.0))

    def test_to_list(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ])

        data = image.to_list();

        self.assertEqual(data[0], RGB(0.0, 0.0, 1.0))
        self.assertEqual(data[1], RGB(1.0, 0.0, 1.0))
        self.assertEqual(data[2], RGB(0.0, 1.0, 1.0))
        self.assertEqual(data[3], RGB(1.0, 0.0, 1.0))

    def test_mapping_get(self):
        colors = [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]

        image = Image.from_list(2, 2, colors)

        self.assertEqual(image[0, 0], colors[0])
        self.assertEqual(image[1, 0], colors[1])
        self.assertEqual(image[0, 1], colors[2])
        self.assertEqual(image[1, 1], colors[3])

    def test_mapping_set(self):
        colors = [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]

        image = Image(2, 2)

        image[0, 0] = colors[0]
        image[1, 0] = colors[1]
        image[0, 1] = colors[2]
        image[1, 1] = colors[3]

        self.assertEqual(image[0, 0], colors[0])
        self.assertEqual(image[1, 0], colors[1])
        self.assertEqual(image[0, 1], colors[2])
        self.assertEqual(image[1, 1], colors[3])

    def test_mapping_len(self):
        self.assertEqual(len(Image(2, 2)), 4)
        self.assertEqual(len(Image(5, 2)), 10)

    def test_has_transparency(self):
        image = Image(4, 7, RGB(1.0, 1.0, 1.0))

        self.assertFalse(image.has_transparency())
        image.put_pixel(0, 0, RGB(0.0, 0.0, 0.0, 0.0))
        self.assertTrue(image.has_transparency())

    def test_is_grayscale(self):
        image = Image(4, 7, RGB(1.0, 1.0, 1.0))

        self.assertTrue(image.is_grayscale())
        image.put_pixel(0, 0, RGB(1.0, 0.0, 0.0, 1.0))
        self.assertFalse(image.is_grayscale())

    def test_mod_colors(self):
        image = Image(2, 2, RGB(1.0, 1.0, 1.0))

        color = RGB(1.0, 0.0, 0.0)

        image.mod_colors(lambda c: color)

        self.assertEqual(image.get_pixel(0, 0), color)
        self.assertEqual(image.get_pixel(0, 1), color)
        self.assertEqual(image.get_pixel(1, 0), color)
        self.assertEqual(image.get_pixel(1, 1), color)

    def test_mod_colors_enumerated(self):
        image = Image(2, 2, RGB(1.0, 1.0, 1.0))

        colors = [
            [RGB(1.0, 0.0, 0.0), RGB(1.0, 1.0, 0.0)],
            [RGB(1.0, 0.0, 1.0), RGB(1.0, 1.0, 1.0)]
        ]

        image.mod_colors_enumerated(lambda x, y, c: colors[x][y])

        self.assertEqual(image.get_pixel(0, 0), colors[0][0])
        self.assertEqual(image.get_pixel(0, 1), colors[0][1])
        self.assertEqual(image.get_pixel(1, 0), colors[1][0])
        self.assertEqual(image.get_pixel(1, 1), colors[1][1])

    def test_map_colors(self):
        image = Image(2, 2, RGB(1.0, 1.0, 1.0))

        color = RGB(1.0, 0.0, 0.0)

        image_out = image.map_colors(lambda c: color)

        self.assertEqual(image_out.get_pixel(0, 0), color)
        self.assertEqual(image_out.get_pixel(0, 1), color)
        self.assertEqual(image_out.get_pixel(1, 0), color)
        self.assertEqual(image_out.get_pixel(1, 1), color)

    def test_map_colors_enumerated(self):
        image = Image(2, 2, RGB(1.0, 1.0, 1.0))

        colors = [
            [RGB(1.0, 0.0, 0.0), RGB(1.0, 1.0, 0.0)],
            [RGB(1.0, 0.0, 1.0), RGB(1.0, 1.0, 1.0)]
        ]

        image_out = image.map_colors_enumerated(lambda x, y, c: colors[x][y])

        self.assertEqual(image_out.get_pixel(0, 0), colors[0][0])
        self.assertEqual(image_out.get_pixel(0, 1), colors[0][1])
        self.assertEqual(image_out.get_pixel(1, 0), colors[1][0])
        self.assertEqual(image_out.get_pixel(1, 1), colors[1][1])

    def test_is_in_image(self):
        image = Image(2, 2, RGB(1.0, 1.0, 1.0))

        self.assertTrue(image.is_in_image(0, 0))
        self.assertTrue(image.is_in_image(1, 1))
        self.assertFalse(image.is_in_image(-1, -1))
        self.assertFalse(image.is_in_image(2, 2))

    def test_flip_horizontal(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]).flip_horizontal()

        self.assertEqual(image.get_pixel(0, 0), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 0), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(0, 1), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 1), RGB(0.0, 1.0, 1.0))

    def test_flip_vertical(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]).flip_vertical()

        self.assertEqual(image.get_pixel(0, 0), RGB(0.0, 1.0, 1.0))
        self.assertEqual(image.get_pixel(1, 0), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(0, 1), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 1), RGB(1.0, 0.0, 1.0))

    def test_rotate_90(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]).rotate90()

        self.assertEqual(image.get_pixel(1, 0), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 1), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(0, 0), RGB(0.0, 1.0, 1.0))
        self.assertEqual(image.get_pixel(0, 1), RGB(1.0, 0.0, 1.0))

    def test_rotate_180(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]).rotate180()

        self.assertEqual(image.get_pixel(1, 1), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(0, 1), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 0), RGB(0.0, 1.0, 1.0))
        self.assertEqual(image.get_pixel(0, 0), RGB(1.0, 0.0, 1.0))

    def test_rotate_270(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]).rotate270()

        self.assertEqual(image.get_pixel(0, 1), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(0, 0), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 1), RGB(0.0, 1.0, 1.0))
        self.assertEqual(image.get_pixel(1, 0), RGB(1.0, 0.0, 1.0))

    def test_rotate(self):
        image = Image.from_list(2, 2, [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]).rotate(180, 'nearest')

        self.assertEqual(image.get_pixel(1, 1), RGB(0.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(0, 1), RGB(1.0, 0.0, 1.0))
        self.assertEqual(image.get_pixel(1, 0), RGB(0.0, 1.0, 1.0))
        self.assertEqual(image.get_pixel(0, 0), RGB(1.0, 0.0, 1.0))

    def test_resize(self):
        image = Image(2, 3).resize(7, 5)

        self.assertEqual(image.width, 7)
        self.assertEqual(image.height, 5)

    def test_resize_pct(self):
        image = Image(2, 3).resize_pct(200)

        self.assertEqual(image.width, 4)
        self.assertEqual(image.height, 6)

    def test_sobel_edge_detection(self):
        image = Image(2, 3).sobel_edge_detection(False)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_with_jpeg_quality(self):
        image = Image(2, 3).with_jpeg_quality(70)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_random_noise(self):
        image = Image(2, 3).random_noise(0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_random_noise(self):
        image = Image(2, 3)
        image.add_random_noise(0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_salt_n_pepper_noise(self):
        image = Image(2, 3).salt_n_pepper_noise(0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_add_salt_n_pepper_noise(self):
        image = Image(2, 3)
        image.add_salt_n_pepper_noise(0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_gaussian_noise(self):
        image = Image(2, 3).gaussian_noise(0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_add_gaussian_noise(self):
        image = Image(2, 3)
        image.add_gaussian_noise(0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_gaussian_blur(self):
        image = Image(2, 3).gaussian_blur(1, 0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_unsharp(self):
        image = Image(2, 3).unsharp(1, 0.5, 0.5)

        self.assertEqual(image.width, 2)
        self.assertEqual(image.height, 3)

    def test_crop(self):
        image = Image(100, 200)

        cropped = image.crop(0, 0, 10, 20)
        self.assertEqual(cropped.width, 10)
        self.assertEqual(cropped.height, 20)


class TestNumpy(unittest.TestCase):

    def test_to_array(self):
        colors = [
            RGB(0.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(0.0, 1.0, 1.0), RGB(1.0, 0.0, 1.0),
            RGB(1.0, 0.0, 1.0), RGB(1.0, 0.0, 1.0),
        ]

        arr = Image.from_list(2, 3, colors).to_np_array()

        shape = np.shape(arr)
        self.assertEqual(shape, (3, 2, 4))

        self.assertEqual(RGB(arr[0, 0, 0], arr[0, 0, 1], arr[0, 0, 2]), colors[0])
        self.assertEqual(RGB(arr[0, 1, 0], arr[0, 1, 1], arr[0, 1, 2]), colors[1])
        self.assertEqual(RGB(arr[1, 0, 0], arr[1, 0, 1], arr[1, 0, 2]), colors[2])
        self.assertEqual(RGB(arr[1, 1, 0], arr[1, 1, 1], arr[1, 1, 2]), colors[3])
        self.assertEqual(RGB(arr[2, 0, 0], arr[2, 0, 1], arr[2, 0, 2]), colors[4])
        self.assertEqual(RGB(arr[2, 1, 0], arr[2, 1, 1], arr[2, 1, 2]), colors[5])

    def test_to_array_color_types(self):
        self.assertEqual(np.shape(Image(1, 2).to_np_array('hsl')), (2, 1, 3))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('hsla')), (2, 1, 4))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('hsv')), (2, 1, 3))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('hsva')), (2, 1, 4))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('yuv')), (2, 1, 3))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('yuva')), (2, 1, 4))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('rgb')), (2, 1, 3))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('rgba')), (2, 1, 4))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('srgb')), (2, 1, 3))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('srgba')), (2, 1, 4))
        self.assertEqual(np.shape(Image(1, 2).to_np_array('gray')), (2, 1, 1))

    def test_to_array_data_types(self):
        self.assertEqual(Image(1, 2).to_np_array().dtype, np.float32)
        self.assertEqual(Image(1, 2).to_np_array(data_type='float32').dtype, np.float32)
        self.assertEqual(Image(1, 2).to_np_array(data_type='float64').dtype, np.float64)
        self.assertEqual(Image(1, 2).to_np_array(data_type='uint8').dtype, np.uint8)
        self.assertEqual(Image(1, 2).to_np_array(data_type='uint16').dtype, np.uint16)
        self.assertEqual(Image(1, 2).to_np_array(data_type='uint32').dtype, np.uint32)
        self.assertEqual(Image(1, 2).to_np_array(data_type='bool').dtype, bool)

        self.assertEqual(Image(1, 2).to_np_array(data_type=np.float32).dtype, np.float32)
        self.assertEqual(Image(1, 2).to_np_array(data_type=np.float64).dtype, np.float64)
        self.assertEqual(Image(1, 2).to_np_array(data_type=np.uint8).dtype, np.uint8)
        self.assertEqual(Image(1, 2).to_np_array(data_type=np.uint16).dtype, np.uint16)
        self.assertEqual(Image(1, 2).to_np_array(data_type=np.uint32).dtype, np.uint32)
        self.assertEqual(Image(1, 2).to_np_array(data_type=np.bool_).dtype, bool)
        self.assertEqual(Image(1, 2).to_np_array(data_type=bool).dtype, bool)
