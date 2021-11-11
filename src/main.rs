/* practice05_ndarray by rust(cargo)
 * 		written by Matsumoto Kazuki
 *
 * 行列ライブラリの練習
 *
 */

extern crate ndarray;
//extern crate ndarray_linalg;

use ndarray::*;
//use ndarray_linalg::*;

fn main()
{
//行列要素による初期化
	let matrix1 = arr2(&[
								[1.0, 2.0, 3.0],
								[4.0, 5.0, 6.0],
								[7.0, 8.0, 9.0]
							]);
	println!("\n＊行列要素による初期化");
	println!("{:?}", matrix1);

//イテレーターと式による初期化
	let matrix2 = Array::<f64, _>::from_shape_fn( (4, 4), |(i, j)| { (i + j) as f64} );
	println!("\n＊行列のイテレーターと式による初期化");
	println!("{:?}", matrix2);

//行列 0 初期化
	let mut matrix2 = Array::<f64, _>::zeros((4, 4));
	println!("\n＊行列 0 初期化");
	println!("{:?}", matrix2);

//行列に代入
	matrix2[[1, 1]] = 2.0;
	println!("\n＊行列代入1");
	println!("{:?}", matrix2);

	matrix2[[0, 0]] = 1.0;
	matrix2[[2, 2]] = 3.0;
	matrix2[[3, 3]] = 4.0;
	println!("\n＊行列代入2");
	println!("{:?}", matrix2);

//行列の全要素に対する演算
	let mut matrix1 = Array::<f64, _>::from_elem( (4, 4), 2.0);

	println!("\n＊行列に対する演算[+]");
	println!("{:?}", &matrix1+4.0 as f64);
	println!("\n＊行列に対する演算[-]");
	println!("{:?}", &matrix1-2.0 as f64);
	println!("\n＊行列に対する演算[*]");
	println!("{:?}", &matrix1*4.0 as f64);
	println!("\n＊行列に対する演算[/]");
	println!("{:?}", &matrix1/2.0 as f64);

//行列積1：行列型(1行・1列)との積
	let matrix1 = arr2(&[
								[1.0, 2.0, 3.0],
								[4.0, 5.0, 6.0],
								[7.0, 8.0, 9.0]
							]);
	let matrix3 = arr1(&[1.0, 2.0, 3.0]);
//	let vec1 :Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];

	let result1 = matrix1.dot(&matrix3);
	let result2 = matrix3.dot(&matrix1);
	println!("\n＊行列積１");
	println!("{:?}\n{:?}", result1, result2);

//行列積2：ベクトルと行列の積(行列型で返却)
	let vec1 :Vec<f64> = vec![3.0, 2.0, 1.0];

	let result1 = matrix1.dot(&(Array1::from(vec1.clone()).into_shape(3).unwrap()));
	let result2 = Array1::from(vec1.clone()).into_shape(3).unwrap().dot(&matrix1);
	println!("\n＊行列積２");
	println!("{:?}\n{:?}", result1, result2);

//行列積3：ベクトルと行列の積(ベクトルで返却)
	let vec1 :Vec<f64> = vec![1.0, 1.0, 1.0];

	let result1 = matrix1.dot(&(Array1::from(vec1.clone()).into_shape(3).unwrap())).to_vec();
	let result2 = Array1::from(vec1.clone()).into_shape(3).unwrap().dot(&matrix1).to_vec();
	println!("\n＊行列積３");
	println!("{:?}\n{:?}", result1, result2);

//行列積４：行列同士の積
	let matrix1 = arr2(&[
								[1.0, 2.0, 3.0],
								[4.0, 5.0, 6.0],
								[7.0, 8.0, 9.0]
							]);
	let matrix2 = arr2(&[
								[9.0, 6.0, 3.0],
								[8.0, 5.0, 2.0],
								[7.0, 4.0, 1.0]
							]);

	let result1 = matrix1.dot(&matrix2);
	println!("\n＊行列積４");
	println!("{:?}", result1);


//アダマール積：行列の要素同士の積１
	let matrix1 = Array::<f64, _>::from_elem( (4, 4), 1.0);
	let matrix2 = Array::<f64, _>::from_elem( (4, 4), 2.0);

	let result1 = &matrix1 * &matrix2;
	println!("\n＊アダマール積１");
	println!("{:?}", result1);

//アダマール積：行列の要素同士の積２(Broadcasting)
	let matrix1 = Array::<f64, _>:: from_elem( (4, 4), 1.0);
	let vec1 :Vec<f64> = vec![2.0, 3.0, 4.0, 5.0];
	let result1 = &matrix1 * &(Array1::from(vec1).into_shape(4).unwrap());
	println!("\n＊アダマール積２");
	println!("{:?}", result1);

}
