// Traditional clocks are increasingly uncommon, but most can still read rotating hands of hours, minutes, and seconds.
// Create function ​clockHandAngles(seconds)​ that, given the number of seconds since 12:00:00, will print the angles (in degrees) of the hour, minute and second hands. As a quick review, there are 360 degrees in a full arc rotation. Treat “straight-up” 12:00:00 as 0 degrees for all hands.

function clockHandAngles(sec){
    let secHand = sec % 60;
    let minHand = sec % 3600 - secHand;
    let hours = sec - minHand;

    secHand *= 6;
    minHand = minHand/60 * 6;
    hours *= hours/1440 * 15;

    if ( hours > 360 ){
        hours = hours % 360;
    }
    console.log( secHand+'-deg sec, '+ minHand+'-deg min, '+hours+'-deg hrs');
    return secHand, minHand, hours
}

clockHandAngles(48093);