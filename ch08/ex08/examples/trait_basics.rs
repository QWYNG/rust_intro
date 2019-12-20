#[derive(Debug, Clone)]
struct CartesianCoord {
    x: f64,
    y: f64,
}

struct PolarCoord {
    r: f64,
    theta: f64,
}

struct Matrix([[f64; 2]; 2]);

trait LinearTransform: Coordinates {
    fn transform(self, matrix: &Matrix) -> Self;

    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
    {
        self.transform(&Matrix([
            [theta.cos(), -theta.sin()],
            [theta.sin(), -theta.cos()],
        ]))
    }
}

trait Coordinates {
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

impl Coordinates for CartesianCoord {
    fn to_cartesian(self) -> CartesianCoord {
        self
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        cart
    }
}

impl LinearTransform for CartesianCoord {
    fn transform(mut self, matrix: &Matrix) -> Self {
        let x = self.x;
        let y = self.y;
        let m = matrix.0;

        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }
}

impl LinearTransform for PolarCoord {
    fn transform(mut self, matrix: &Matrix) -> Self {
        let x = self.x;
        let y = self.y;
        let m = matrix.0;

        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }

    fn rotate(mut self, theta: f64) -> Self {
        self.theta += theta;
        self
    }
}

impl Coordinates for PolarCoord {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        PolarCoord {
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

impl Coordinates for (f64, f64) {
    fn to_cartesian(self) -> CartesianCoord {
        CartesianCoord {
            x: self.0,
            y: self.1,
        }
    }

    fn from_cartesian(cart: CartesianCoord) -> Self {
        (cart.x, cart.y)
    }
}

fn main() {
    let point = (1.0, 1.0);

    let c = point.to_cartesian();
    println!("x= {}, y = {}", c.x, c.y);

    let p = PolarCoord::from_cartesian(c);
    println!("r = {}, θ = {}", p.r, p.theta);

    print_point(p);
    let p = (1.0, 0.0).to_cartesian();
    print_point(p.rotate(std::f64::consts::PI))
}

fn print_point(point: impl Coordinates) {
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y)
}
