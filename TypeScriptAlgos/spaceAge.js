//? Given an age in seconds, calculate how old someone would be on: (Round to 2 decimal places)
// Mercury: orbital period 0.2408467 Earth years
// Venus: orbital period 0.61519726 Earth years
// Earth: orbital period 1.0 Earth years, 365.25 Earth days, or 31557600 seconds
// Mars: orbital period 1.8808158 Earth years
// Jupiter: orbital period 11.862615 Earth years
// Saturn: orbital period 29.447498 Earth years
// Uranus: orbital period 84.016846 Earth years
// Neptune: orbital period 164.79132 Earth years
// So if you were told someone were 1,000,000,000 seconds old, youtsc  should be able to say that they're 31.69 Earth-years old.
function age(planet, seconds) {
    var years = seconds / 31557600;
    if (planet === 'mercury') {
        years /= 0.2408467;
    }
    if (planet === 'venus') {
        years /= 0.61519726;
    }
    if (planet === 'mars') {
        years /= 1.8808158;
    }
    if (planet === 'jupiter') {
        years /= 11.862615;
    }
    if (planet === 'saturn') {
        years /= 29.447498;
    }
    if (planet === 'uranus') {
        years /= 84.016846;
    }
    if (planet === 'neptune') {
        years /= 164.79132;
    }
    years = Math.round(years * 100) / 100;
    return years;
}
age('earth', 1000000000); // -> 31.69
age('mercury', 2134835688); // -> 280.88
age('venus', 189839836); // -> 9.78
age('mars', 2129871239); // -> 35.88
age('jupiter', 901876382); // -> 2.41
age('saturn', 2000000000); // -> 2.15
age('uranus', 1210123456); // -> 0.46
age('neptune', 1821023456); // -> 0.35
