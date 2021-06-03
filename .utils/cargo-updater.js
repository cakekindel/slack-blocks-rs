const {pick} = require('./common');

/// # Public
module.exports = {
    readVersion: cargoTomlContents => getVer(cargoTomlContents),
    writeVersion: (cargoTomlContents, version) => setVer(cargoTomlContents, version),
};

/// # Not Public

// Regex pattern that matches against the version property in a Cargo.toml file,
//     with the first capture group containing the version number.
const tomlVersionPattern = /['"]?version['"]?\s*=\s*['"]([\d\.]+)['"]/mi;

// read the "version" property in Cargo.toml
const getVer = toml => pick(tomlVersionPattern.exec(toml), 1);

// update the "version" property with a new version
const setVer = (toml, ver) => toml.replace(getVer(toml), ver);

/// # Tests

// poor man's testing framework
const regexTests = [
    '[package] version = "0.4.0"',
    `[package]
version = "0.4.0"`,
    'version = "0.4.0"',
    'version      =\'0.4.0\'',
    '"version"="0.4.0"',
    '        \'version\'="0.4.0"',
];

regexTests.forEach(toml => {
    if (getVer(toml) !== '0.4.0') {
        throw new Error('in ' + toml + ' expected 0.4.0 got ' + getVer(toml));
    }
});
