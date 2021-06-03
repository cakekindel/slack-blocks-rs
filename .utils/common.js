// returns a bool indicating whether a given val is null or undefined
const isNullish = val => typeof val === 'null' || typeof val === 'undefined';

// safely access a property
const pick = (val, prop) => isNullish(val) ? undefined : val[prop];

module.exports = {isNullish, pick};
