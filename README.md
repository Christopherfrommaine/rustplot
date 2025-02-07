This is a personal-use plotting util for rust.

It uses ascii/unicode art to display graphs, plots, and images in the standard output, in a string, or saved to a file.

# Basic Syntax / Example Usage

Each plot file contains a public function by the name of that plot. For example,
```/src/plots/array_plot.rs
pub fn array_plot<T>(data: &Vec<Vec<T>>) -> ArrayPlotBuilder<T>
```
creates an ArrayPlotBuilder instance.

You can then set options for it such as title, axes, output size, and more (depending on the type of plot). Finally, you can call .print() or .as_string() or .pyplot() (for only some plot types) to print it to the standard output, or return the plot as a string, or display an interactive window with matplotlib, respectively.:
```
use cgrustplot::{
      plots::array_plot::{array_plot, bin_arr},
      helper::charset::gradient_chars,
};

let my_data: Vec<Vec<f64>> = ...;  // table of sin(x + y)

array_plot(&bin_arr(&my_data, 4)) // Bins the array to have four values
.set_title("A Plot of my Data:".to_string())  // Sets the title of the plot
.set_axes(true) // Turns on the axes for the plot
.set_chars(gradient_chars::shade_chars()) // uses unicode shade characters
.print()  // Displays the plot
```

And the output of the above code is this plot:

*NOTE: some plots may not display correctly without a monowidth font, such as on github*

```
A Plot of my Data:
10.0000┼▓██▓  ░██▓   ▓██░  ▓██░  ░██▓
       │██▓  ░██▓   ▓██░  ▓██░  ░██▓
       │█▓  ░██▓   ▓██░  ▓██░  ░██▓  ░
 6.6667┼▓  ░██▓   ▓██░  ▓██░  ░██▓  ░█
       │  ░██▓   ▓██░  ▓██░  ░██▓  ░██
       │ ░██▓   ▓██░  ▓██░  ░██▓  ░███
 3.3333┼░██▓   ▓██░  ▓██░  ░██▓  ░███░
       │██▓   ▓██░  ▓██░  ░██▓  ░███░
       │█▓   ▓██░  ▓██░  ░██▓  ░███░
 0.0000┼▓   ▓██░  ▓██░  ░██▓  ░███░  ▓
       └┼──────┼──────┼──────┼──────┼─
        0.000  7.241  14.483 21.724 28.966
```
# Plot Types
In no particular order, here are some of the various types of plots that can be used
(more plots than listed here may already be available. Check the /src/plots/ for a full list while in development).

## Array Plot
Filename: array_plot.rs

Displays a table with varying brightness across it's characters.
Takes in a `Vec<Vec<T>>`, representing the brightness value at each point.

The dimensions of the output string are equal to the dimensions of the input table, so if you give a 15x12 grid of values, the output will be about 15x12 characters in size. (Note that axes and title will change this).

### Options
`title: Option<String>` Sets the title to be displayed above the output. Default is None, which has no displayed title.

`axes: bool` Selects whether to turn on or off the axes. Axes display in units of the number of characters. Default is true

`chars: Vec<String>` The character set to be used. Defaults are based on `rustplot::helper::charset::gradient_chars`, depending on the number of distinct values in the table.

`bins: Option<u32>` Only for plots of f64. It bins close-together datapoints and plots based on those. Not so much a plot option as much as a transformation, as it actually creates a new struct of `<u32>` instead of `<f64>`.

### Example

Code:
```
let data: Vec<Vec<i32>> = (-5..=5)
	.map(|i: i32|
		(-15..=15).map(|j: i32|
			i.pow(2) + j.pow(2)
		).collect()
	).collect();

array_plot(&data)
.print()
```

Output:
```
   │@%%#**++=---:::::::---=++**#%%@   
9.9┼@%##**+==--::.....::--==+**##%@
   │@%##*++=--::.......::--=++*##%@
   │@%##*+==-::...   ...::-==+*##%@
6.6┼@%#**+==-::..     ..::-==+**#%@
   │%%#**+=--::..     ..::--=+**#%%
   │@%#**+==-::..     ..::-==+**#%@
3.3┼@%##*+==-::...   ...::-==+*##%@
   │@%##*++=--::.......::--=++*##%@
   │@%##**+==--::.....::--==+**##%@
0.0┼@%%#**++=---:::::::---=++**#%%@
   └┼──────┼──────┼──────┼──────┼──
    0.000  7.233  14.467 21.700 28.933
```

## Function Plot
Filename: func_plot.rs

Displays the output of a numerically-valued function over a domain.
Takes in a `Fn(U) -> V` for types which can be cast to and from f64, respectively.

### Options
`domain: (f64, f64)` Sets the domain (i.e. min and max x values) over which to plot the function. If no domain is selected, `rustplot::helper::func_plot_domain::determine_plot_domain` will be used as a default. By a variety of heuristic methods, it is usually able to determine a domain over which some useful behavior can be observed.

`range: (f64, f64)` Sets the range (i.e. min and max y values) over which to plot the function. Default comes from the min and max values of the function within it's domain.

`domain_padding: f64` Pads the domain by some percentage. For example, with padding of 0.01, the domain (0, 10) gets turned into (-0.1, 10.1).

`range_padding: f64` Pads the range by some percentage.

`size: (u32, u32)` Sets the size of the output of the plot, measured in number of characters.

`title: Option<String>`

`axes: bool`

### Example

Code:
```
let f = |x: f64| x.powi(3);

function_plot(f)
.set_size((30, 10))
.print();
```

Output:
```
1.8476┼                            ╱    
      │                           ╱     
      │                         ╱‾      
0.6159┼                       _―        
      │                _____―‾          
      │        _―‾‾‾‾‾‾                 
-0.616┼      ╱‾                         
      │    ╱‾                           
      │   ╱                             
-1.848┼ ╱_                              
      └┼──────┼──────┼──────┼──────┼─   
       -1.39  -0.72  -0.05  0.621  1.290
```

## Scatter Plot
Filename: scatter_plot.rs

Displays a scatter plot from a given set of points `Vec<(f64, f64)>`.

### Options
`range: ((f64, f64), (f64, f64))` Sets the domain as well as range (i.e. min and max x and values) over which to plot the function. Sorry about the inconsistency between range here and in other places. It may be fixed in the future.

`padding: f64` Pads both the domain and range by some percentage.

`size: (u32, u32)`

`title: Option<String>`

`axes: bool`

`chars: (Vec<char>, (u32, u32))` The character set to be used. Defaults are based on `rustplot::helper::charset::subdiv_chars`, depending on the number of unique points that would be overwritten at each size. (i.e. it tries not to show two points in only a single character). You must specify the dimension of each character. For example, braille characters allow you to plot a grid of 2x4 dots for each character, so you must pass in (2, 4) in addition to the character set.

### Example
Code:
```
// Generate some random data points within ((0, 60), (0, 30))
let mut rng: StdRng = SeedableRng::seed_from_u64(0);
let data: Vec<(f64, f64)> = (0..100).map(|_|
		(rng.gen_range(0.0..60.0), rng.gen_range(0.0..30.0))
	).collect();

  

scatter_plot(&data).set_size((30, 10))
.set_chars((dots_two_by_four(), (2, 4)))
.set_range(((0., 60.), (0., 30.)))
.set_padding(0.)
.print();
```

Output:
```
30┼⠔⡈⢀⠀⠂⠀⠀⠂⠀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠂⢁⠀⠀⠂⠀⠀⠀⡀⠀⠀⠄
  │⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⢀⠀⠀⠁⠐⠀⠁⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠐
  │⠀⠀⠀⠄⠀⠄⠀⠠⠀⠀⡀⠈⠀⠈⠀⠀⠀⠀⠠⠀⠀⠀⠈⠀⠀⠀⠀⠀⠀⠀
20┼⠄⠀⠁⠀⠀⠀⠁⠀⠡⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠀⠀⠀⠀⠈⠀⠀⠐⠀⠀
  │⠀⠂⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⠀⠀⠀⠀⠀⡀⠀⠀⠀⠀⠀⠀⠀⠂⠀⠀
  │⢀⠀⠀⠀⠀⠁⠣⠀⠀⠀⠠⠄⠀⠀⠰⠀⠀⢂⠀⠀⠀⠀⠀⠁⠀⠀⣀⠀⠀⠀
10┼⠠⢀⠀⠀⠅⠠⠀⠀⠐⠀⠀⠀⠀⠀⠀⠀⠀⠁⠈⠀⠀⠀⠀⠠⠀⠐⠀⣀⠠⠠
  │⠀⠀⠈⠂⠀⠀⢀⠡⠀⠀⠀⠀⠀⠀⠀⠠⠀⠀⠀⠀⡀⠠⠀⠀⠀⠀⠀⠀⠀⠀
  │⠀⠐⠀⠀⠐⠀⠀⠀⠀⠀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠄⠀⠀⢀⠄⠁⠄⠄⠀⠀⠀
 0┼⠀⢀⡀⠄⠁⠀⢀⠀⠀⠀⠀⠀⠀⠈⠀⢀⠀⠀⠀⠀⠀⢀⠄⠀⠀⠁⠀⠀⠀⠀
  └┼──────┼──────┼──────┼──────┼─
   0.000  14.483 28.966 43.448 57.931
```


## Region Plot
Filename: Region_plot.rs

Displays the region over which a predicate `pred: Fn(f64, f64) -> bool` is true.

### Options
`range: ((f64, f64), (f64, f64))` Sets the domain as well as range (i.e. min and max x and values) over which to plot the predicate

`padding: f64`

`size: (u32, u32)`

`title: Option<String>`

`axes: bool`

### Example
Code:
```
let p = |x: f64, y: f64| (x.powi(2) + y.powi(2)).sqrt() <= 0.7;

region_plot(p)
.set_domain_and_range(((-1., 1.), (-1., 1.)))
.set_size((30, 10))
.print();
```

Output:
```
1.20┼                                 
    │                                 
    │           ▗▄▄▄▄▄▖               
0.40┼       ▗▄▄▄▟█████▙▄▄▄▖           
    │      ▗▟█████████████▙▖          
    │      ▐███████████████▌          
-0.4┼      ▝▜█████████████▛▘          
    │       ▝▀▀▀▜█████▛▀▀▀▘           
    │           ▝▀▀▀▀▀▘               
-1.2┼                                 
    └┼──────┼──────┼──────┼──────┼─   
     -1.20  -0.62  -0.04  0.538  1.117
```


# Release Notes
## Features
For many plot types, I added support for plotting with matplotlib through python. For example, if you have a scatter plot, instead of displaying it in terminal, you can use the .pyplot() method to create an interactive window, which is often much more useful.

## Bugfixes and Updates
Lots of cleanup from the initial release, all of it mostly internal. Most plots were taking clones of all the input data, or owning it outright, and many of these were easily changed to references with lifetimes instead.
Axes were also inconsistent and poorly implemented and overall not reliable. I want this plotting software to work well enough that it can actually be relied upon without having to go into another program just to make sure that the axes marking are correct.