/// ASCII art banner for suntheme CLI

const Y: &str = "\x1b[93m"; // bright yellow
const R: &str = "\x1b[0m";  // reset

pub const TAGLINE: &str = "   Automatic theme switching\n      powered by the sun\n";

/// Print the welcome banner with colored egg yolk
pub fn print_welcome() {
    println!();
    println!("             \\   |   /");
    println!("          '   .---.   '");
    println!("            / {Y}o   o{R} \\");
    println!("       ___.'.  ---  .'.___");
    println!("      '-------------------------'");
    println!();
    println!("         {Y}s u n t h e m e{R}");
    println!();
    println!("{}", TAGLINE);
}
