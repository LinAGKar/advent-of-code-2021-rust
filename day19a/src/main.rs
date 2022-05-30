use std::io::Read;
use std::ops::{Add,Index,IndexMut,Mul,Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Vec4 {
    elems: [i16; 4],
}

impl Index<usize> for Vec4 {
    type Output = i16;

    fn index(&self, i: usize) -> &Self::Output {
        &self.elems[i]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elems[i]
    }
}

impl Sub<Vec4> for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Self::Output {
        let mut result = Vec4 { elems: [0; 4] };
        for i in 0..4 {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Self::Output {
        let mut result = Vec4 { elems: [0; 4] };
        for i in 0..4 {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Vec4 {
    fn to_translation_matrix(self) -> Mat4 {
        Mat4 {
            elems: [
                [1, 0, 0, self[0]],
                [0, 1, 0, self[1]],
                [0, 0, 1, self[2]],
                [0, 0, 0, 1],
            ],
        }
    }

    fn in_range(self) -> bool {
        self.elems.iter().all(|&elem| elem.abs() <= 1000)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Mat4 {
    elems: [[i16; 4]; 4],
}

impl Index<usize> for Mat4 {
    type Output = [i16; 4];

    fn index(&self, i: usize) -> &Self::Output {
        &self.elems[i]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elems[i]
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Self::Output {
        let mut result = Mat4 { elems: [[0; 4]; 4] };
        for c in 0..4 {
            for l in 0..4 {
                result[l][c] = (0..4).map(|i| self[l][i] * other[i][c]).sum();
            }
        }
        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Self::Output {
        let mut result = Vec4 { elems: [0; 4] };
        for i in 0..4 {
            result[i] = (0..4).map(|j| self[i][j] * other[j]).sum();
        }
        result
    }
}

fn fit_scanner(scanner1: &Vec<Vec4>, scanner2: &Vec<Vec4>, orientation_matrices: &Vec<Mat4>) -> Option<Mat4> {
    // Works in scanner1's frame of reference

    for &m in orientation_matrices {
        let rotated_scanner2: Vec<_> = scanner2.iter().map(move |&beacon| m * beacon).collect();

        for &beacon2 in &rotated_scanner2 {
            for &beacon1 in scanner1 {
                let diff = beacon1 - beacon2;
                let scanner2_center = Vec4 { elems: [0, 0, 0, 1] } + diff;

                let mut scanner1_covered = vec![false; scanner1.len()];
                if rotated_scanner2.iter().map(move |&beacon| beacon + diff).all(|beacon2| {
                    if !beacon2.in_range() {
                        true
                    } else if let Some(index) = scanner1.iter().position(|&beacon1| beacon1 == beacon2) {
                        scanner1_covered[index] = true;
                        true
                    } else {
                        false
                    }
                }) && scanner1.iter().enumerate().all(|(n, &beacon)| {
                    scanner1_covered[n] || !(beacon - scanner2_center).in_range()
                }) && scanner1_covered.into_iter().filter(|&i| i).count() >= 12 {
                    return Some(diff.to_translation_matrix() * m);
                }
            }
        }
    }

    None
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let scanners: Vec<Vec<_>> = input.split("\n\n").map(|scanner| {
        scanner.lines().skip(1).map(|line| {
            let mut parts = line.split(',').map(|coord| coord.parse::<i16>().unwrap());
            Vec4 { elems: [parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap(), 1] }
        }).collect()
    }).collect();

    let mut scanner_orientations = vec![None; scanners.len()];
    // Identity matrix, as orientations will be in scanners[0]'s frame of reference
    scanner_orientations[0] = Some(Mat4 {
        elems: [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
        ],
    });

    // Each possible facing of the x-axis
    let facing_matrices = [
        Mat4 {
            elems: [
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [-1, 0, 0, 0],
                [0, -1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [0, 1, 0, 0],
                [-1, 0, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [0, -1, 0, 0],
                [1, 0, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [0, 0, 1, 0],
                [0, 1, 0, 0],
                [-1, 0, 0, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [0, 0, -1, 0],
                [0, 1, 0, 0],
                [1, 0, 0, 0],
                [0, 0, 0, 1],
            ],
        },
    ];

    // Each possible rotation around the x-axis
    let rotation_matrices = [
        Mat4 {
            elems: [
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [1, 0, 0, 0],
                [0, -1, 0, 0],
                [0, 0, -1, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [1, 0, 0, 0],
                [0, 0, -1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 1],
            ],
        },
        Mat4 {
            elems: [
                [1, 0, 0, 0],
                [0, 0, 1, 0],
                [0, -1, 0, 0],
                [0, 0, 0, 1],
            ],
        },
    ];

    let orientation_matrices: Vec<_> = rotation_matrices.into_iter().flat_map(move |m1| {
        facing_matrices.into_iter().map(move |m2| m2 * m1)
    }).collect();

    let mut fitted_scanners = vec![0];

    for i in 0..scanners.len() {
        let n = fitted_scanners[i];
        let scanner1 = &scanners[n];

        for (m, scanner2) in scanners.iter().enumerate() {
            if m == n || scanner_orientations[m].is_some() {
                continue;
            }

            if let Some(orientation2) = fit_scanner(scanner1, scanner2, &orientation_matrices) {
                scanner_orientations[m] = Some(scanner_orientations[n].unwrap() * orientation2);
                fitted_scanners.push(m);
            }
        }
    }

    let beacons: std::collections::HashSet<_> =
        scanners.into_iter().zip(scanner_orientations).flat_map(|(beacons, orientation)| {
            beacons.into_iter().map(move |beacon| orientation.unwrap() * beacon)
        }).collect();

    println!("{}", beacons.len());
}
