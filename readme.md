# CLI App: Joke Generator

The **Joke Generator** is a command-line application that allows users to generate jokes from different categories. The application fetches jokes from an external API and provides them in a user-friendly format through the terminal.

## Installation

To use the **Joke Generator** CLI app, you need to have [Rust](https://www.rust-lang.org/) installed on your system. If you don't have Rust installed, you can follow the installation instructions from the Rust website.

    cargo install --path . && joke-generator

## Usage

The **Joke Generator** CLI app provides a few simple commands that allow you to interact with it:

## 1. Generate a Random Joke

To generate a random joke, simply run the following command:

    joke-generator


The app will fetch a random joke from the API and display it in the terminal.

## 2. Generate a Joke from a Specific Category

If you want to get a joke from a specific category, you can use the `--category` or `-c` option:

    joke-generator --category Programming

Replace `Programming` with the category of your choice. The app will fetch a joke from the specified category and display it in the terminal.

## 4. Generate Jokes in German

By default, the app fetches jokes in English. However, you can get jokes in other languages using the `--lang` or `l` option:

    joke-generator --lang de

Replace `de` with the language code of your choice. The app will fetch a joke in the specified language and display it in the terminal.

##  5.  Display help message

    joke-generator --help

# Examples

Here are some examples of how to use the **Joke Generator** CLI app:

## Generate a random joke

    joke-generator


## Get a joke from the "Programming" category

    joke-generator --category Programming


## Support

If you encounter any issues, have questions, or want to contribute to the project, feel free to open an issue or submit a pull request on the GitHub repository: [https://github.com/sirloko/joke-generator](https://github.com/sirloko/joke-generator)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

Thank you for using the **Joke Generator** CLI app. We hope you have fun generating jokes and sharing laughter! If you have any feedback or suggestions, don't hesitate to reach out. Happy joking! 😄

## Credits

The **Joke Generator** CLI app fetches jokes from the [JokeAPI](https://sv443.net/jokeapi/v2) provided by [sv443](https://sv443.net/). We would like to express our gratitude to sv443 for offering this fantastic API, which allows us to bring laughter and humor to our users.

The JokeAPI provides a wide range of jokes in various categories, making it an excellent source for generating jokes on demand. Without this amazing API, the **Joke Generator** app wouldn't be possible.

If you'd like to explore more about the JokeAPI or show your support to sv443, visit their website [JokeAPI](https://sv443.net/jokeapi/v2) and [sv443.net](https://sv443.net/).

Thank you, sv443, for spreading laughter with your JokeAPI! 🎉😄


