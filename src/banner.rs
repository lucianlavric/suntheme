/// ASCII art banner for suntheme CLI

pub const BANNER: &str = r#"
             \   |   /
          '   .---.   '
            / o   o \
       ___.'.  ---  .'.___
      '-------------------------'

         s u n t h e m e

"#;

pub const TAGLINE: &str = "   Automatic theme switching\n      powered by the sun\n";

/// Print the welcome banner
pub fn print_welcome() {
    println!("{}", BANNER);
    println!("{}", TAGLINE);
}
