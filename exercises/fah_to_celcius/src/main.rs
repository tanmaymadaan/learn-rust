fn main() {
    // fahrenheit to celcius
    let fahrenheit = 100.0;
    let celcius = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("{} fahrenheit is {} celcius", fahrenheit, celcius);

    // celcius to fahrenheit
    let celcius = 20.0;
    let fahrenheit = (celcius * (9.0 / 5.0)) + 32.0;
    println!("{} celcius is {} fahrenheit", celcius, fahrenheit);
}
