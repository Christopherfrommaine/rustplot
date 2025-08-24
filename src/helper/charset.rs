//! Helper file for storing sets of characters used for plotting graphs

pub const NULL_STR: &str = "�";
pub const NULL_CHR: char = '�';

/// Used for displaying axes on a plot.
/// 
/// # Characters
/// 
/// VERTICAL:    '│'
/// 
/// HORIZONTAL:  '─'
/// 
/// CROSS:       '┼'
/// 
/// CORNER:      '└'
/// 
pub mod axes_chars {
    pub const VERTICAL: char = '│';
    pub const HORIZONTAL: char = '─';
    pub const CROSS: char = '┼';
    pub const CORNER: char = '└';
}

/// Used for shading a charachter to a specific brightness.
/// 
/// # Characters
/// 
/// binary_chars():       " █"
/// 
/// shade_chars():        " ░▒▓█"
/// 
/// ascii_chars():        " .:-=+*#%@"
/// 
/// ascii_chars_large():  " .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
/// 
/// # Notes
/// 
/// Contains charachter gradients of multiple sizes, of the form Vec<String>
pub mod gradient_chars {
    // Vec<String> instead of Vec<char> for future ideas:
    // e.g. using ANSI escape codes for color / bold if the terminal supports it
    // Charachter sets largely based on https://paulbourke.net/dataformats/asciiart/
    pub fn binary_chars() -> Vec<String> {[" ", "█"].iter().map(|&s| s.into()).collect()}
    pub fn shade_chars() -> Vec<String> {[" ", "░", "▒", "▓", "█"].iter().map(|&s| s.into()).collect()}
    pub fn ascii_chars() -> Vec<String> {[" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"].iter().map(|&s| s.into()).collect()}
    pub fn ascii_chars_large() -> Vec<String> {[" ", ".", "'", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_", "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p", "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "@", "$"].iter().map(|&s| s.into()).collect()}
}

/// Used for subdividing a single charachter into multiple individual binary pieces.
/// 
/// # Characters
/// 
/// dots_one_by_one():    " ●"
/// 
/// blocks_two_by_two():  " ▘▖▌▝▀▞▛▗▚▄▙▐▜▟█"
/// 
/// dots_two_by_foour():  "⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿"
/// 
/// # Notes
/// 
/// Ordered in terms of binary from left to right, top to bottom
pub mod subdiv_chars {
    pub fn dots_one_by_one() -> Vec<char> {" ●".chars().collect()}
    pub fn blocks_two_by_two() -> Vec<char> {" ▘▖▌▝▀▞▛▗▚▄▙▐▜▟█".chars().collect()}
    pub fn dots_two_by_four() -> Vec<char>    {"⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿".chars().collect()}
    pub fn blocks_two_by_three() -> Vec<char> {" 🬀🬃🬄🬏🬐🬓▌🬁🬂🬅🬆🬑🬒🬔🬕🬇🬈🬋🬌🬖🬗🬚🬛🬉🬊🬍🬎🬘🬙🬜🬝🬞🬟🬢🬣🬭🬮🬱🬲🬠🬡🬤🬥🬯🬰🬳🬴🬦🬧🬩🬪🬵🬶🬹🬺▐🬨🬫🬬🬷🬸🬻█".chars().collect()}
}

/// Used for displaying continuous single lines. 
/// 
/// # Characters
/// 
/// `"_―‾╱/╲\|"`
/// 
pub mod line_chars {
    pub const FLAT_LOW: char = '_';
    pub const FLAT_MED: char = '―';
    pub const FLAT_HIGH: char = '‾';

    pub const UP_ONE: char = '╱';
    pub const UP_TWO: char = '/';
    pub const DOWN_ONE: char = '╲';
    pub const DOWN_TWO: char = '\\';

    pub const VERTICAL: char = '|';
}

pub mod vec_chars {
    pub const ARROWS: &'static str = "→↘↓↙←↖↑↗";
}