pub(crate) const NULL_STR: &str = "�";

/// Contains charachter gradients of multiple sizes, of the form Vec<String>
pub(crate) mod gradient_chars {
    // Vec<String> instead of Vec<char> for future ideas:
    // e.g. using ANSI escape codes for color / bold if the terminal supports it
    // Charachter sets largely based on https://paulbourke.net/dataformats/asciiart/
    pub fn binary_chars() -> Vec<String> {[" ", "█"].iter().map(|&s| s.into()).collect()}
    pub fn shade_chars() -> Vec<String> {[" ", "░", "▒", "▓", "█"].iter().map(|&s| s.into()).collect()}
    pub fn ascii_chars() -> Vec<String> {[" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"].iter().map(|&s| s.into()).collect()}
    pub fn ascii_chars_large() -> Vec<String> {[" ", ".", "'", "`", "^", "\"", ",", ":", ";", "I", "l", "!", "i", ">", "<", "~", "+", "_", "-", "?", "]", "[", "}", "{", "1", ")", "(", "|", "\\", "/", "t", "f", "j", "r", "x", "n", "u", "v", "c", "z", "X", "Y", "U", "J", "C", "L", "Q", "0", "O", "Z", "m", "w", "q", "p", "d", "b", "k", "h", "a", "o", "*", "#", "M", "W", "&", "8", "%", "B", "@", "$"].iter().map(|&s| s.into()).collect()}
}

/// Contains plotting charachters for subdividing a single charachter into multiple individual pieces
pub(crate) mod subdiv_chars {
    pub fn dots_one_by_one() -> Vec<char> {" ●".chars().collect()}
    pub fn blocks_two_by_two() -> Vec<char> {" ▘▖▌▝▀▞▛▗▚▄▙▐▜▟█".chars().collect()}
    pub fn dots_two_by_four() -> Vec<char> {"⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿".chars().collect()}
}
