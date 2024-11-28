#[allow(unused_imports)] // imports are used, but doesn't detect it?
use crate::plots::array_plot::*;

#[test]
fn array_plot_test_1 () {
    let matrix = [[0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0], [0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0], [0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0], [0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0], [1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1]];
    let vec_mat: Vec<Vec<u8>> = matrix.into_iter().map(|i| i.into_iter().collect()).collect();
    let left = array_plot(&vec_mat).as_string();
    let right = 
"       █       
      ███      
     █  ██     
    ████ ██    
   █   █  ██   
  ███ ████ ██  
 █  █    █  ██ 
██████  ████ ██";
    println!("{}\n --- \n{}", left, right);
    assert_eq!(left, right);
}

#[test]
fn array_plot_test_2 () {
    let matrix: [[i8; 15]; 8] = [[0, 0, 0, 3, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 0], [0, 0, 0, 0, -5, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0], [0, 0, 0, 9, 0, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0], [0, 0, 0, 3, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0], [0, 0, 0, 1, 2, 0, 0, 1, 0, 0, 1, 1, 0, 9, 0], [0, 0, 1, 1, 1, 0, 2, 1, 1, 1, 0, 1, 1, 0, 0], [0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0], [1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1]];
    let vec_mat: Vec<Vec<i8>> = matrix.into_iter().map(|i| i.into_iter().collect()).collect();
    let left = array_plot(&vec_mat).as_string();
    let right = 
":::#:::=:::::+:
:::: :===::::::
:::@:=::==:::::
:::#====:==::::
:::=+::=::==:@:
::===:+===:==::
:=::=::::=::==:
======::====:==";

    println!("{}\n --- \n{}", left, right);
    assert_eq!(left, right);
}

#[test]
fn array_plot_test_3 () {
    let vec_mat: Vec<Vec<i32>> = (0..10).into_iter().map(|i| (0..50).into_iter().map(|j| i + j).collect()).collect();
    let left = array_plot(&vec_mat).as_string();
    let right = 
" .'^\",:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkha
.'^\",:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao
'^\",:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#
^\",:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#M
\",:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#MW
,:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#MW&
:;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#MW&8
;l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#MW&8B
l!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#MW&8B@
!i><~_-?][{1)(|/tfjrnuvczXUJCLQOZmwqdbkhao#MW&8B@$";

    println!("{}\n --- \n{}", left, right);
    assert_eq!(left, right);
}

#[test]
fn array_plot_test_4() {
    let vec_mat: Vec<Vec<f64>> = vec![vec![f64::NAN]];
    let left = array_plot(&vec_mat).as_string();
    let right = String::from("�");

    println!("{}\n --- \n{}", left, right);
    assert_eq!(left, right);
}

#[test]
fn density_plot_test_1() {
    let vec_mat: Vec<Vec<f64>> =
        (0..50).map(|i| {
            (0..50).map(|j|
                (((i - 25) * (i - 25) + (j - 25) * (j - 25)) as f64).sqrt()
            ).collect()
        }).collect();

    let left = density_plot(&vec_mat, 8).as_string();
    let right = 
"@@@@@@@%%%%%%%%%%*****************%%%%%%%%%%@@@@@@
@@@@@@%%%%%%%%***********************%%%%%%%%@@@@@
@@@@@%%%%%%%***************************%%%%%%%@@@@
@@@@%%%%%%%************+++++************%%%%%%%@@@
@@@%%%%%%**********+++++++++++++**********%%%%%%@@
@@%%%%%%********+++++++++++++++++++********%%%%%%@
@%%%%%%*******+++++++++++++++++++++++*******%%%%%%
%%%%%%*******+++++++++++++++++++++++++*******%%%%%
%%%%%******++++++++++=========++++++++++******%%%%
%%%%******++++++++===============++++++++******%%%
%%%%*****+++++++===================+++++++*****%%%
%%%*****+++++++=====================+++++++*****%%
%%******++++++=========-----=========++++++******%
%%*****++++++=======-----------=======++++++*****%
%*****++++++======---------------======++++++*****
%*****+++++======-----------------======+++++*****
%****+++++======-------------------======+++++****
*****+++++=====-------.......-------=====+++++****
*****++++=====------...........------=====++++****
****+++++=====-----.............-----=====+++++***
****+++++====-----...............-----====+++++***
****++++=====-----......   ......-----=====++++***
****++++=====----.....       .....----=====++++***
***+++++====-----.....       .....-----====+++++**
***+++++====-----....         ....-----====+++++**
***+++++====-----....         ....-----====+++++**
***+++++====-----....         ....-----====+++++**
***+++++====-----.....       .....-----====+++++**
****++++=====----.....       .....----=====++++***
****++++=====-----......   ......-----=====++++***
****+++++====-----...............-----====+++++***
****+++++=====-----.............-----=====+++++***
*****++++=====------...........------=====++++****
*****+++++=====-------.......-------=====+++++****
%****+++++======-------------------======+++++****
%*****+++++======-----------------======+++++*****
%*****++++++======---------------======++++++*****
%%*****++++++=======-----------=======++++++*****%
%%******++++++=========-----=========++++++******%
%%%*****+++++++=====================+++++++*****%%
%%%%*****+++++++===================+++++++*****%%%
%%%%******++++++++===============++++++++******%%%
%%%%%******++++++++++=========++++++++++******%%%%
%%%%%%*******+++++++++++++++++++++++++*******%%%%%
@%%%%%%*******+++++++++++++++++++++++*******%%%%%%
@@%%%%%%********+++++++++++++++++++********%%%%%%@
@@@%%%%%%**********+++++++++++++**********%%%%%%@@
@@@@%%%%%%%************+++++************%%%%%%%@@@
@@@@@%%%%%%%***************************%%%%%%%@@@@
@@@@@@%%%%%%%%***********************%%%%%%%%@@@@@";
    
    println!("{}\n --- \n{}", left, right);
    assert_eq!(left, right);
}