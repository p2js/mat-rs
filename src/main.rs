use mat_rs::*;

fn main() {
    let _33a = mat![
        7, -4, 2;
        3, 1, -5;
        2, 2, -5
    ]
    .determinant();

    let _33b = mat![
       1, -6, -7;
       1, -4,  7;
      -1, -3, -6
    ]
    .determinant();

    let _33c = mat![
      -1, -1, -1;
       4,  5, -3;
      -1, -6,  3
    ]
    .determinant();

    let _44a = mat![
      4,  3,  2, 2;
      0,  1, -3, 3;
      0, -1,  3, 3;
      0,  3,  1, 1
    ]
    .reduced_row_echelon_form();

    let _44b = mat![
       2,  3,  3, 1;
       1,  5,  4, 3;
       4,  6,  8, 5;
      -2, -3, -3, 4
    ]
    .determinant(); //this is correct??????????????/

    let _m1 = mat![
          4, -4,   3, 2, 1;
          0,  6,   2, 1, 7;
          4,  3, -12, 4, 8;
        -12,  3,   4, 4, 3;
          1,  1, -12, 0, 1
    ];

    println!("{}", _44a.unwrap());
}
