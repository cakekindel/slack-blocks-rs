// Slack raw HTML string from stdin
let contents = '';

// Read `contents` from stdin
process.stdin.setEncoding('utf8');

process.stdin.on('readable', () => {
    let chunk = process.stdin.read();
    while (chunk) {
        contents += chunk;
        chunk = process.stdin.read();
    }
});

process.stdin.on('end', () => {
    // array of String -> String map
    // functions that the stdin will
    // be piped through
    const transformers = [
        // Separate sentences with newline
        appendNewlineToPeriods,

        // ex. <a src="/link"> -> <a src="https://api.slack.com/link">
        qualifyRootLinks,

        // ex. <a src="#anchor"> -> <a src="CURRENT_PAGE#anchor">
        qualifyAnchors,

        // ex. <a src="/link">Text</a> -> [Text 🔗] [Text 🔗]: https://api.slack.com/link
        escapeLinks,
        
        // Move MD link definitions to the bottom
        moveLinkDefsToBottom,

        // ex. <code>blah</code> -> `blah`
        escapeCode,

        // ex. <strong>blah</strong> -> **blah**
        escapeStrong,

        // Prepend each line with ///
        docComment,
    ];

    // Flow stdin through each transformer
    const output = transformers.reduce((c, t) => t(c), contents);

    process.stdout.write(output);
});

// Give some meaning to the Array#sort
// integer return values
const ord = {
    aFirst: -1,
    bFirst: 1,
    equal: 0,
};

const helpers = {
// Test if a string is a markdown link definition,
// for example: '[Test]: ...' would return true.
    
//  isLink :: String -> Boolean
    isLink: str => (/\[.*?\]\:/i).test(str),

// Accepts a Regex pattern, replacement string, and input String
// Returns the input string with String#replace applied.
    
//  replace :: (RegExp, String) -> String -> String
    replace:   (pat,   replace) => contents => {
        const output = contents.replace(pat, replace);
        return output;
    },

// Accepts an input String and String -> String fn,
// Returns the input string with the mapping fn applied to each line.
    
//  eachLine :: (String -> String) -> String -> String
    eachLine:   map                => contents =>
        contents
            .split('\n')
            .map(map)
            .join('\n'),
}

// # Transformers

//    qualifyRootLinks :: String -> String
const qualifyRootLinks = helpers.replace(
  /<a href\="\/(.*?)">/gi,
  '<a href\="https://api.slack.com/$1">'
);

//    qualifyAnchors :: String -> String
const qualifyAnchors = helpers.replace(
  /<a href\="#(.*?)">/gi,
  `<a href\="CURRENT_PAGE$1">`
);

//    escapeLinks :: String -> String
const escapeLinks = helpers.replace(
  /<a href\="(.*?)">(.*?)<\/a>/gi,
  '[$2 🔗]%%[$2 🔗]: https://api.slack.com$1\n%%'
);

//    escapeCode :: String -> String
const escapeCode = helpers.replace(/<code>(.*?)<\/code>/gi, '`$1`');

//    escapeStrong :: String -> String
const escapeStrong = helpers.replace(/<strong>(.*?)<\/strong>/gi, '**$1**');

//    appendNewlineToPeriods :: String -> String
const appendNewlineToPeriods = helpers.replace(/\./gi, '.\n');

//    moveLinkDefsToBottom :: String -> String
const moveLinkDefsToBottom = c =>
    c.split(/%%/g)
        .sort((a, b) => helpers.isLink(a)
              ? ord.bFirst
              : helpers.isLink(b)
                  ? ord.aFirst
                  : ord.equal
        )
        .join('');

//    docComment :: String -> String
const docComment = helpers.eachLine(c => '/// ' + c);
