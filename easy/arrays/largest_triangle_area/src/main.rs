// You have a list of points in the plane. Return the area of the largest triangle that can be formed by any 3 of the points.

// Example:
// Input: points = [[0,0],[0,1],[1,0],[0,2],[2,0]]
// Output: 2
// Explanation:
// The five points are show in the figure below. The red triangle is the largest.

// Notes:

// 3 <= points.length <= 50.
// No points will be duplicated.
//  -50 <= points[i][j] <= 50.
// Answers within 10^-6 of the true value will be accepted as correct.

struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut result = 0.0;

        for i in 0..points.len() - 2 {
            for j in 1..points.len() - 1 {
                for k in 2..points.len() {
                    let ab = Self::length(&points[i], &points[j]);
                    let bc = Self::length(&points[j], &points[k]);
                    let ac = Self::length(&points[i], &points[k]);
                    let s = (ab + bc + ac) / 2.0;
                    let area = (s * (s - ab) * (s - bc) * (s - ac)).sqrt();
                    if area > result {
                        result = area;
                    }
                }
            }
        }

        result
    }

    fn length(a: &Vec<i32>, b: &Vec<i32>) -> f64 {
        (((a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])) as f64).sqrt()
    }
}

fn main() {
    assert_eq!(
        Solution::largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.0
    );
}
