/*
  What this script does: Convert slack documentation HTML to rustdoc comments.

  How to use this script:
  - In Slack API Docs, e.g. https://api.slack.com/reference/block-kit/composition-objects#option
    1. right click > inspect element on description, e.g. for `text` (A text object that defines...)
    2. select <td> wrapping the paragraph, right click > copy inner HTML
    3. in shell, do `echo 'PASTE HTML' | node .utils/slack-dom-to-rustdoc.js`
    4. Rustdoc comments are printed to standard output
*/

// Slack raw HTML string from stdin
let contents = '';

// Read `contents` from stdin
process.stdin.setEncoding('utf8');

process.stdin.on('readable', () => {
    const storeNextChunk = () => {
      const chunk = process.stdin.read();

      if (chunk) { contents += chunk; }

      return chunk ? storeNextChunk() : {};
    };

    storeNextChunk();
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

        // ex. <em>blah</em> -> _blah_
        escapeItalic,

        removeEmptyLines,

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
    // isLink :: String -> Boolean
    isLink: str => (/\[.*?\]\:/i).test(str),

    // Accepts a Regex pattern, replacement string, and input String
    // Returns the input string with String#replace applied.
    // replace :: (RegExp, String) -> String -> String
    replace:   (pat,   replace) => contents => {
        const output = contents.replace(pat, replace);
        return output;
    },

    // Accepts an input String and String -> String fn,
    // Returns the input string with the mapping fn applied to each line.
    // eachLine :: (String -> String) -> String -> String
    eachLine: map => contents => contents
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
  '[$2 🔗]%%[$2 🔗]: $1\n%%'
);

//    escapeCode :: String -> String
const escapeCode = helpers.replace(/<code>(.*?)<\/code>/gi, '`$1`');

//    escapeStrong :: String -> String
const escapeStrong = helpers.replace(/<strong>(.*?)<\/strong>/gi, '**$1**');

//    escapeItalic :: String -> String
const escapeItalic = helpers.replace(/<em>(.*?)<\/em>/gi, '_$1_');

//    appendNewlineToPeriods :: String -> String
const appendNewlineToPeriods = helpers.replace(/\. /gi, '.\n');

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

//    removeEmptyLines :: String -> String
const removeEmptyLines = contents => contents.split('\n').filter(s => s !== '').join('\n');

//    docComment :: String -> String
const docComment = helpers.eachLine(c => '/// ' + c);
