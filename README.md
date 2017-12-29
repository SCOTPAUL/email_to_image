# email\_to\_image for Google Fonts

Program for converting email addresses to images for the purpose of avoiding automatic email harvesting by spammers.

Similar to sites like [this](http://www.generateit.net/email-to-image/index.php) and [this](http://safemail.justlikeed.net/) but with access to any font on [Google Fonts](https://fonts.google.com/).

## Usage

```
USAGE:
    email_to_image [OPTIONS] <EMAIL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --background-color <BG_COLOR>    Sets color of background in #rrggbb(aa) format
    -f, --font <FONT>                    Sets the text font
    -o, --output <OUTPUT_FILE>           Sets the output filename
    -s, --size <FONT_SIZE>               Sets the font size
    -c, --text-color <TEXT_COLOR>        Sets color of text in #rrggbb(aa) format

ARGS:
    <EMAIL>    Sets the email address
```

## Example

Say you want a font which will match the default font and text/background colours used in the footer of a [Materialize](http://materializecss.com/footer.html) styled website.

The default font size is `16`, background colour is `#ee6e73`, foreground colour is `#ffffff`, and font is [Roboto](https://fonts.google.com/specimen/Roboto).

Using the command:

```bash
./email_to_image -b "#ee6e73" -c "#ffffff" -s 16 -f "Roboto" bob@thebuilder.com
```

will produce an `email.png` file containing the required email image.

## Comparison

- The original text:

![Original Text](https://raw.githubusercontent.com/SCOTPAUL/email_to_image/master/imgs/text.png)

- The generated text:

![Original Text](https://raw.githubusercontent.com/SCOTPAUL/email_to_image/master/imgs/generated.png)

## Building 

1. Clone this repository
2. [Install rustc and cargo if necessary](https://www.rust-lang.org/en-US/install.html)
3. `cd` to the cloned repository and run `cargo build --release`
4. An executable will be produced at `email_to_image/target/release/email_to_image`
