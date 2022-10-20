use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};
use termion::{color, clear};
use ndarray::{arr2, Array};

#[allow(non_upper_case_globals)]

const n:usize = 10;

fn cube_coords() -> Vec<Vec<f64>> {
    let mut cube_coords = Vec::new();
    for x in 0..10*n{
        for y in 0..10*n{
            for z in 0..10*n{

                if 4*n <= x && x <= 5*n{
                    if y == 4*n-1{
                        if 4*n <= z && z <= 5*n{
                            cube_coords.push(vec![x as f64, y as f64, z as f64, 0f64])
                        }
                    }
                }


                if 4*n <= y && y <= 5*n{
                    if x == 4*n-1{
                        if 4*n <= z && z <= 5*n{
                            cube_coords.push(vec![x as f64, y as f64, z as f64, 1f64])
                        }
                    }
                }

                if 4*n <= y && y <= 5*n{
                    if z == 4*n-1{
                        if 4*n <= x && x <= 5*n{
                            cube_coords.push(vec![x as f64, y as f64, z as f64, 2f64])
                        }
                    }
                }

                if 4*n <= x && x <= 5*n{
                    if y == 5*n{
                        if 4*n <= z && z <= 5*n{
                            cube_coords.push(vec![x as f64, y as f64, z as f64, 3f64])
                        }
                    }
                }


                if 4*n <= y && y <= 5*n{
                    if x == 5*n{
                        if 4*n <= z && z <= 5*n{
                            cube_coords.push(vec![x as f64, y as f64, z as f64, 4f64])
                        }
                    }
                }

                if 4*n <= y && y <= 5*n{
                    if z == 5*n{
                        if 4*n <= x && x <= 5*n{
                            cube_coords.push(vec![x as f64, y as f64, z as f64, 5f64])
                        }
                    }
                }
            }
        }
    }    

    cube_coords
}

#[allow(non_snake_case)]
fn update_R3(frame_n: i64, cube: Vec<Vec<f64>>) -> Vec<Vec<i64>> {
    let mut R3 = Vec::new();

    let alpha = 0.01 * frame_n as f64;
    let beta  = 0.01 * frame_n as f64 * 2f64; 
    let gamma = 0.01 * frame_n as f64 * 3f64;

    let rotation_matrix = arr2(&[                                       
[alpha.cos() * beta.cos(), alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos(), alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin()],
[alpha.sin() * beta.cos(), alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() * gamma.cos(), alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin()],
[-(beta.sin())           , beta.cos() * gamma.sin()                                          , beta.cos() * gamma.cos()                                          ]
]);


    for point in cube{
        let mut v = Vec::new();
        let point_coords = vec![point[0], point[1], point[2]];
        rotation_matrix.dot(&Array::from(point_coords)).map(|x| v.push((*x).round() as i64));
        v.push(point[3] as i64);
        R3.push(v);
    }

    R3
}

#[allow(non_snake_case)]
fn R3_to_R2(mut R3: Vec<Vec<i64>>) -> Vec<Vec<String>>{
    let mut R2 = vec![vec![" ".to_string() ; n*20] ; n*20];
    R3.sort_by_key(|v| (*v)[2]);
    for v in R3{
        if v[0] + 10*n as i64 >= 0 && v[1] + 10*n as i64 >= 0 { 
            let colorful_point = match v[3]{
                0 => format!("{}@{}", color::Fg(color::LightRed), color::Fg(color::Reset)),
                1 => format!("{}@{}", color::Fg(color::LightBlue), color::Fg(color::Reset)),
                2 => format!("{}@{}", color::Fg(color::LightGreen), color::Fg(color::Reset)),
                3 => format!("{}@{}", color::Fg(color::LightMagenta), color::Fg(color::Reset)),
                4 => format!("{}@{}", color::Fg(color::LightYellow), color::Fg(color::Reset)),
                5 => format!("{}@{}", color::Fg(color::LightCyan), color::Fg(color::Reset)),

                _ => panic!()
            };
            R2[(v[0] + 10*n as i64) as usize][(v[1] + 10*n as i64) as usize] = colorful_point;
        }
    }

    R2
}

#[allow(non_snake_case)]
fn vec_to_string(R2: Vec<Vec<String>>) -> String{
    let mut frame: String = "".to_string();

    for y in R2{
        for x in y{
            frame.push_str(&x);
        }
        frame.push_str("\n");
    }
    frame
}

fn main(){
    let cube: Vec<Vec<f64>> = cube_coords();
    let mut frame_n = 1;
    loop{
        print!("{}", clear::All);
        print!("{}\r", vec_to_string(R3_to_R2(update_R3(frame_n, cube.clone()))));
        io::stdout().flush().unwrap(); 
        sleep(Duration::from_millis(1));
        frame_n += 1; 

        
    }
}