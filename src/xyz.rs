use num::traits::Float;

use std::ops::{Add, Sub, Mul, Div};

use {Color, Rgb, Luma, Lab, Lch, Hsv, Hsl, ColorSpace, Mix, Shade, clamp};

use tristimulus::{X_N, Y_N, Z_N};

///The CIE 1931 XYZ color space with an alpha component.
///
///XYZ links the perceived colors to their wavelengths and simply makes it
///possible to describe the way we see colors as numbers. It's often used when
///converting from one color space to an other, and requires a standard
///illuminant and a standard observer to be defined.
///
///Conversions and operations on this color space assumes the CIE Standard
///Illuminant D65 as the white point, and the 2° standard colorimetric
///observer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Xyz<T: Float = f32> {
    ///X is the scale of what can be seen as a response curve for the cone
    ///cells in the human eye. It goes from 0.0 to 1.0.
    pub x: T,

    ///Y is the luminance of the color, where 0.0 is black and 1.0 is white.
    pub y: T,

    ///Z is the scale of what can be seen as the blue stimulation. It goes
    ///from 0.0 to 1.0.
    pub z: T,

    ///The transparency of the color. 0.0 is completely transparent and 1.0 is
    ///completely opaque.
    pub alpha: T,
}

impl<T: Float> Xyz<T> {
    ///CIE XYZ.
    pub fn xyz(x: T, y: T, z: T) -> Xyz<T> {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: T::one(),
        }
    }

    ///CIE XYZ and transparency.
    pub fn xyza(x: T, y: T, z: T, alpha: T) -> Xyz<T> {
        Xyz {
            x: x,
            y: y,
            z: z,
            alpha: alpha,
        }
    }
}

impl<T: Float> ColorSpace for Xyz<T> {
    fn is_valid(&self) -> bool {
        self.x >= T::zero() && self.x <= T::one() && self.y >= T::zero() &&
        self.y <= T::one() && self.z >= T::zero() && self.z <= T::one() &&
        self.alpha >= T::zero() && self.alpha <= T::one()
    }

    fn clamp(&self) -> Xyz<T> {
        let mut c = *self;
        c.clamp_self();
        c
    }

    fn clamp_self(&mut self) {
        self.x = clamp(self.x, T::zero(), T::one());
        self.y = clamp(self.y, T::zero(), T::one());
        self.z = clamp(self.z, T::zero(), T::one());
        self.alpha = clamp(self.alpha, T::zero(), T::one());
    }
}

impl<T: Float> Mix for Xyz<T> {
    type Scalar = T;

    fn mix(&self, other: &Xyz<T>, factor: T) -> Xyz<T> {
        let factor = clamp(factor, T::zero(), T::one());

        Xyz {
            x: self.x + factor * (other.x - self.x),
            y: self.y + factor * (other.y - self.y),
            z: self.z + factor * (other.z - self.z),
            alpha: self.alpha + factor * (other.alpha - self.alpha),
        }
    }
}

impl<T: Float> Shade for Xyz<T> {
    type Scalar = T;

    fn lighten(&self, amount: T) -> Xyz<T> {
        Xyz {
            x: self.x,
            y: self.y + amount,
            z: self.z,
            alpha: self.alpha,
        }
    }
}

impl<T: Float> Default for Xyz<T> {
    fn default() -> Xyz<T> {
        Xyz::xyz(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Float> Add<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn add(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            alpha: self.alpha + other.alpha,
        }
    }
}

impl<T: Float> Add<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn add(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x + c,
            y: self.y + c,
            z: self.z + c,
            alpha: self.alpha + c,
        }
    }
}

impl<T: Float> Sub<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn sub(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            alpha: self.alpha - other.alpha,
        }
    }
}

impl<T: Float> Sub<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn sub(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x - c,
            y: self.y - c,
            z: self.z - c,
            alpha: self.alpha - c,
        }
    }
}

impl<T: Float> Mul<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn mul(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            alpha: self.alpha * other.alpha,
        }
    }
}

impl<T: Float> Mul<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn mul(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x * c,
            y: self.y * c,
            z: self.z * c,
            alpha: self.alpha * c,
        }
    }
}

impl<T: Float> Div<Xyz<T>> for Xyz<T> {
    type Output = Xyz<T>;

    fn div(self, other: Xyz<T>) -> Xyz<T> {
        Xyz {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            alpha: self.alpha / other.alpha,
        }
    }
}

impl<T: Float> Div<T> for Xyz<T> {
    type Output = Xyz<T>;

    fn div(self, c: T) -> Xyz<T> {
        Xyz {
            x: self.x / c,
            y: self.y / c,
            z: self.z / c,
            alpha: self.alpha / c,
        }
    }
}

from_color!(to Xyz from Rgb, Luma, Lab, Lch, Hsv, Hsl);

impl<T: Float> From<Rgb<T>> for Xyz<T> {
    fn from(rgb: Rgb<T>) -> Xyz<T> {
        Xyz {
            x: rgb.red * T::from(0.4124).unwrap() + rgb.green * T::from(0.3576).unwrap() + rgb.blue * T::from(0.1805).unwrap(),
            y: rgb.red * T::from(0.2126).unwrap() + rgb.green * T::from(0.7152).unwrap() + rgb.blue * T::from(0.0722).unwrap(),
            z: rgb.red * T::from(0.0193).unwrap() + rgb.green * T::from(0.1192).unwrap() + rgb.blue * T::from(0.9505).unwrap(),
            alpha: rgb.alpha,
        }
    }
}

impl<T: Float> From<Luma<T>> for Xyz<T> {
    fn from(luma: Luma<T>) -> Xyz<T> {
        Xyz {
            x: T::zero(),
            y: luma.luma,
            z: T::zero(),
            alpha: luma.alpha,
        }
    }
}

impl<T: Float> From<Lab<T>> for Xyz<T> {
    fn from(lab: Lab<T>) -> Xyz<T> {
        Xyz {
            x: T::from(X_N).unwrap() * f_inv((T::one() / T::from(116.0).unwrap()) *
                (lab.l * T::from(100.0).unwrap() + T::from(16.0).unwrap()) +
                (T::one() / T::from(500.0).unwrap()) * lab.a * T::from(128.0).unwrap()),
            y: T::from(Y_N).unwrap() * f_inv((T::one() / T::from(116.0).unwrap()) *
                (lab.l * T::from(100.0).unwrap() + T::from(16.0).unwrap())),
            z: T::from(Z_N).unwrap() * f_inv((T::one() / T::from(116.0).unwrap()) *
                (lab.l * T::from(100.0).unwrap() + T::from(16.0).unwrap()) -
                (T::one() / T::from(200.0).unwrap()) * lab.b * T::from(128.0).unwrap()),
            alpha: lab.alpha,
        }
    }
}

impl<T: Float> From<Lch<T>> for Xyz<T> {
    fn from(lch: Lch<T>) -> Xyz<T> {
        Lab::from(lch).into()
    }
}

impl<T: Float> From<Hsv<T>> for Xyz<T> {
    fn from(hsv: Hsv<T>) -> Xyz<T> {
        Rgb::from(hsv).into()
    }
}

impl<T: Float> From<Hsl<T>> for Xyz<T> {
    fn from(hsl: Hsl<T>) -> Xyz<T> {
        Rgb::from(hsl).into()
    }
}


fn f_inv<T: Float>(t: T) -> T {
    //(6/29)^2
    let c_6_o_29_p_2: T = T::from(0.04280618311).unwrap();

    if t > T::from(6.0 / 29.0).unwrap() {
        t * t * t
    } else {
        T::from(3.0).unwrap() * c_6_o_29_p_2 * (t - T::from(4.0 / 29.0).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::Xyz;
    use ::Rgb;

    #[test]
    fn red() {
        let a = Xyz::from(Rgb::linear_rgb(1.0, 0.0, 0.0));
        let b = Xyz::xyz(0.41240, 0.21260, 0.01930);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn green() {
        let a = Xyz::from(Rgb::linear_rgb(0.0, 1.0, 0.0));
        let b = Xyz::xyz(0.35760, 0.71520, 0.11920);
        assert_approx_eq!(a, b, [x, y, z]);
    }

    #[test]
    fn blue() {
        let a = Xyz::from(Rgb::linear_rgb(0.0, 0.0, 1.0));
        let b = Xyz::xyz(0.18050, 0.07220, 0.95050);
        assert_approx_eq!(a, b, [x, y, z]);
    }
}
